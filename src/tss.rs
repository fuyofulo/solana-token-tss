#![allow(non_snake_case)]

use curv::elliptic::curves::{Ed25519, Point, Scalar};
use multi_party_eddsa::protocols::{musig2, ExpandedKeyPair};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer, Signature};
use solana_sdk::hash::Hash;
use solana_sdk::transaction::Transaction;
use solana_sdk::message::Message;
use solana_client::rpc_client::RpcClient;
use spl_associated_token_account;
use spl_token;

use crate::error::Error;
use crate::serialization::{AggMessage1, SecretAggStepOne, PartialSignature, Serialize};
use crate::token::get_ata_address;

/// Create the aggregate public key from a list of public keys
/// Pass key=None if you don't care about the coefficient (typically for key aggregation only)
/// Pass key=Some(pubkey) if you want to get the coefficient for a specific key in the aggregation
pub fn key_agg(keys: Vec<Pubkey>, key: Option<Pubkey>) -> Result<musig2::PublicKeyAgg, Error> {
    // Convert Solana pubkeys to Ed25519 points
    let convert_keys = |k: Pubkey| {
        Point::from_bytes(&k.to_bytes()).map_err(|e| Error::PointDeserializationFailed {
            error: e,
            field_name: "keys",
        })
    };
    
    // Convert all keys to Ed25519 points
    let keys: Vec<Point<Ed25519>> = keys.into_iter().map(convert_keys).collect::<Result<_, _>>()?;
    
    // If no specific key is provided for coefficient calculation, use the first key
    let key = key.map(convert_keys).unwrap_or_else(|| Ok(keys[0].clone()))?;
    
    // Perform MuSig2 key aggregation
    musig2::PublicKeyAgg::key_aggregation_n(keys, &key).ok_or(Error::KeyPairIsNotInKeys)
}

/// Generate Message1 which contains nonce, public nonce, and commitment to nonces
/// This is the first step in the MPC signing process
pub fn step_one(keypair: Keypair) -> (AggMessage1, SecretAggStepOne) {
    let extended_keypair = ExpandedKeyPair::create_from_private_key(keypair.secret().to_bytes());
    // we don't really need to pass a message here.
    let (private_nonces, public_nonces) = musig2::generate_partial_nonces(&extended_keypair, None);

    (
        AggMessage1 { sender: keypair.pubkey(), public_nonces: public_nonces.clone() },
        SecretAggStepOne { private_nonces, public_nonces },
    )
}

/// Generate partial signature for token transfer (Step 2 of MPC)
#[allow(clippy::too_many_arguments)]
pub fn step_two_token(
    keypair: Keypair,
    mint: Pubkey,
    amount: u64,
    decimals: u8,
    to: Pubkey,
    recent_block_hash: Hash,
    keys: Vec<Pubkey>,
    first_messages: Vec<AggMessage1>,
    secret_state: SecretAggStepOne,
    rpc_client: &RpcClient,
) -> Result<PartialSignature, Error> {
    let other_nonces: Vec<_> = first_messages.into_iter().map(|msg1| msg1.public_nonces.R).collect();

    // Generate the aggregate key together with the coefficient of the current keypair
    let aggkey = key_agg(keys, Some(keypair.pubkey()))?;
    let agg_bytes = aggkey.agg_public_key.to_bytes(true);
    let mut pubkey_bytes = [0u8; 32];
    pubkey_bytes.copy_from_slice(&agg_bytes);
    let aggpubkey = Pubkey::from(pubkey_bytes);
    let extended_keypair = ExpandedKeyPair::create_from_private_key(keypair.secret().to_bytes());

    // Create the unsigned token transaction
    let mut tx = create_unsigned_token_transaction(mint, amount, decimals, &to, &aggpubkey, rpc_client)?;

    let signer = PartialSigner {
        signer_private_nonce: secret_state.private_nonces,
        signer_public_nonce: secret_state.public_nonces,
        other_nonces,
        extended_keypair,
        aggregated_pubkey: aggkey,
    };
    
    // Sign the transaction using a custom `PartialSigner`, this is required to comply with Solana's API.
    tx.sign(&[&signer], recent_block_hash);
    let sig = tx.signatures[0];
    Ok(PartialSignature(sig))
}

/// Create an unsigned token transfer transaction
pub fn create_unsigned_token_transaction(
    mint: Pubkey,
    amount: u64,
    decimals: u8,
    to: &Pubkey,
    payer: &Pubkey,
    rpc_client: &RpcClient,
) -> Result<Transaction, Error> {
    // Calculate source and destination ATAs
    let source_ata = get_ata_address(payer, &mint);
    let destination_ata = get_ata_address(to, &mint);
    
    let mut instructions = Vec::new();
    
    // Check if destination ATA exists, if not, create it
    if rpc_client.get_account(&destination_ata).is_err() {
        let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
            payer,  // fee payer
            to,     // wallet owner
            &mint,  // mint
            &spl_token::id(),
        );
        instructions.push(create_ata_ix);
    }
    
    // Create the token transfer instruction
    let transfer_ix = spl_token::instruction::transfer_checked(
        &spl_token::id(),
        &source_ata,
        &mint,
        &destination_ata,
        payer,      // authority (aggregated key)
        &[],        // signers (will be filled by MPC)
        amount,
        decimals,
    ).map_err(|e| Error::TokenTransferFailed(format!("Failed to create transfer instruction: {}", e)))?;
    
    instructions.push(transfer_ix);
    
    // Create the message and transaction
    let msg = Message::new(&instructions, Some(payer));
    Ok(Transaction::new_unsigned(msg))
}

/// Aggregate partial signatures and create a final signed token transfer transaction (Step 3 of MPC)
#[allow(clippy::too_many_arguments)]
pub fn sign_and_broadcast_token(
    mint: Pubkey,
    amount: u64,
    decimals: u8,
    to: Pubkey,
    recent_block_hash: Hash,
    keys: Vec<Pubkey>,
    signatures: Vec<PartialSignature>,
    rpc_client: &RpcClient,
) -> Result<Transaction, Error> {
    let aggkey = key_agg(keys.clone(), None)?;
    let agg_bytes = aggkey.agg_public_key.to_bytes(true);
    let mut pubkey_bytes = [0u8; 32];
    pubkey_bytes.copy_from_slice(&agg_bytes);
    let aggpubkey = Pubkey::from(pubkey_bytes);

    // Make sure all the `R`s are the same (first 32 bytes of each signature)
    if !signatures[1..].iter().map(|s| &s.0.as_ref()[..32]).all(|s| s == &signatures[0].0.as_ref()[..32]) {
        return Err(Error::MismatchMessages);
    }

    let deserialize_R = |s: &[u8]| {
        Point::from_bytes(s).map_err(|e| Error::PointDeserializationFailed {
            error: e,
            field_name: "signatures R component",
        })
    };
    
    let deserialize_s = |s: &[u8]| {
        Scalar::from_bytes(s).map_err(|e| Error::ScalarDeserializationFailed {
            error: e,
            field_name: "signatures s component",
        })
    };

    // Deserialize the first signature's R and s components
    let first_sig = musig2::PartialSignature {
        R: deserialize_R(&signatures[0].0.as_ref()[..32])?,
        my_partial_s: deserialize_s(&signatures[0].0.as_ref()[32..])?,
    };

    // Deserialize all other partial s values
    let partial_sigs: Vec<_> = signatures[1..]
        .iter()
        .map(|s| deserialize_s(&s.0.as_ref()[32..]))
        .collect::<Result<_, _>>()?;

    // Add the signatures up using MuSig2 aggregation
    let full_sig = musig2::aggregate_partial_signatures(&first_sig, &partial_sigs);

    // Convert the aggregated signature to Solana format
    let mut sig_bytes = [0u8; 64];
    sig_bytes[..32].copy_from_slice(&*full_sig.R.to_bytes(true));
    sig_bytes[32..].copy_from_slice(&full_sig.s.to_bytes());
    let sig = Signature::from(sig_bytes);

    // Create the same transaction again with the aggregated signature
    let mut tx = create_unsigned_token_transaction(mint, amount, decimals, &to, &aggpubkey, rpc_client)?;
    
    // Insert the recent_block_hash and the signature
    tx.message.recent_blockhash = recent_block_hash;
    assert_eq!(tx.signatures.len(), 1);
    tx.signatures[0] = sig;

    // Verify the resulting transaction is actually valid
    if tx.verify().is_err() {
        return Err(Error::InvalidSignature);
    }
    
    Ok(tx)
}

struct PartialSigner {
    signer_private_nonce: musig2::PrivatePartialNonces,
    signer_public_nonce: musig2::PublicPartialNonces,
    other_nonces: Vec<[Point<Ed25519>; 2]>,
    extended_keypair: ExpandedKeyPair,
    aggregated_pubkey: musig2::PublicKeyAgg,
}

impl solana_sdk::signer::Signer for PartialSigner {
    fn try_pubkey(&self) -> Result<Pubkey, solana_sdk::signer::SignerError> {
        let agg_bytes = self.aggregated_pubkey.agg_public_key.to_bytes(true);
        let mut pubkey_bytes = [0u8; 32];
        pubkey_bytes.copy_from_slice(&agg_bytes);
        Ok(Pubkey::from(pubkey_bytes))
    }

    fn try_sign_message(&self, message: &[u8]) -> Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        let sig = musig2::partial_sign(
            &self.other_nonces,
            self.signer_private_nonce.clone(),
            self.signer_public_nonce.clone(),
            &self.aggregated_pubkey,
            &self.extended_keypair,
            message,
        );
        let mut sig_bytes = [0u8; 64];
        sig_bytes[..32].copy_from_slice(&*sig.R.to_bytes(true));
        sig_bytes[32..].copy_from_slice(&sig.my_partial_s.to_bytes());
        Ok(solana_sdk::signature::Signature::from(sig_bytes))
    }

    fn is_interactive(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::{Keypair, Signer};
    
    #[test]
    fn test_key_aggregation() {
        // Generate test keypairs
        let mut rng = rand07::thread_rng();
        let keypairs: Vec<Keypair> = (0..3).map(|_| Keypair::generate(&mut rng)).collect();
        let pubkeys: Vec<Pubkey> = keypairs.iter().map(|k| k.pubkey()).collect();
        
        // Test key aggregation without specific key
        let agg_result = key_agg(pubkeys.clone(), None);
        assert!(agg_result.is_ok());
        
        let agg_key = agg_result.unwrap();
        let agg_bytes = agg_key.agg_public_key.to_bytes(true);
        let mut pubkey_bytes = [0u8; 32];
        pubkey_bytes.copy_from_slice(&agg_bytes);
        let agg_pubkey = Pubkey::from(pubkey_bytes);
        
        // The aggregated pubkey should be different from any individual pubkey
        assert!(!pubkeys.contains(&agg_pubkey));
        
        // Test key aggregation with specific key
        let agg_result_with_key = key_agg(pubkeys.clone(), Some(pubkeys[0]));
        assert!(agg_result_with_key.is_ok());
    }
    
    #[test]
    fn test_key_aggregation_with_invalid_key() {
        // Generate test keypairs
        let mut rng = rand07::thread_rng();
        let keypairs: Vec<Keypair> = (0..3).map(|_| Keypair::generate(&mut rng)).collect();
        let pubkeys: Vec<Pubkey> = keypairs.iter().map(|k| k.pubkey()).collect();
        
        // Generate a key that's not in the list
        let invalid_key = Keypair::generate(&mut rng).pubkey();
        
        // This should fail because the key is not in the list
        let result = key_agg(pubkeys, Some(invalid_key));
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::KeyPairIsNotInKeys => {},
            _ => panic!("Expected KeyPairIsNotInKeys error"),
        }
    }
    
    #[test]
    fn test_single_key_aggregation() {
        // Test with a single key
        let mut rng = rand07::thread_rng();
        let keypair = Keypair::generate(&mut rng);
        let pubkeys = vec![keypair.pubkey()];
        
        let result = key_agg(pubkeys.clone(), None);
        assert!(result.is_ok());
        
        let agg_key = result.unwrap();
        let agg_bytes = agg_key.agg_public_key.to_bytes(true);
        let mut pubkey_bytes = [0u8; 32];
        pubkey_bytes.copy_from_slice(&agg_bytes);
        let agg_pubkey = Pubkey::from(pubkey_bytes);
        
        // For a single key, the aggregated key should be the same as the original
        assert_eq!(pubkeys[0], agg_pubkey);
    }
}
