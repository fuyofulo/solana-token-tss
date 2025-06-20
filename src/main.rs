use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token, signature::{Signer, Keypair}, hash::Hash};
use solana_sdk::pubkey::Pubkey;
use std::fs;

mod cli;
mod error;
mod tss;
mod serialization;
mod token;

use cli::{Options};
use error::Error;
use serialization::Serialize;

// Helper function to read a keypair from a file
fn read_keypair_file(file_path: &str) -> Result<Keypair, Error> {
    let keypair_bytes = fs::read(file_path)
        .map_err(|e| Error::FileReadError(format!("Failed to read keypair file {}: {}", file_path, e)))?;
    
    if keypair_bytes.len() == 64 {
        // Raw 64-byte keypair
        Keypair::from_bytes(&keypair_bytes)
            .map_err(Error::WrongKeyPair)
    } else {
        // Try to parse as base58 string
        let keypair_string = String::from_utf8(keypair_bytes)
            .map_err(|e| Error::FileReadError(format!("Invalid UTF-8 in keypair file {}: {}", file_path, e)))?;
        let decoded = bs58::decode(keypair_string.trim())
            .into_vec()
            .map_err(Error::BadBase58)?;
        Keypair::from_bytes(&decoded)
            .map_err(Error::WrongKeyPair)
    }
}

fn main() -> Result<(), Error> {
    let opts = Options::parse();

    match opts {
        Options::Generate => {
            let keypair = solana_sdk::signature::Keypair::generate(&mut rand07::thread_rng());
            println!("secret share (base58): {}", keypair.to_base58_string());
            println!("public key: {}", keypair.pubkey());
        }

        Options::Balance { address, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            let balance = rpc_client
                .get_balance(&address)
                .map_err(Error::BalaceFailed)?;
            println!("The balance of {} is: {}", address, balance);
        }

        Options::Airdrop { to, amount, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            let amount = native_token::sol_to_lamports(amount);
            let sig = rpc_client
                .request_airdrop(&to, amount)
                .map_err(Error::AirdropFailed)?;
            println!("Airdrop transaction ID: {}", sig);

            let recent_hash = rpc_client
                .get_latest_blockhash()
                .map_err(Error::RecentHashFailed)?;
            rpc_client
                .confirm_transaction_with_spinner(&sig, &recent_hash, rpc_client.commitment())
                .map_err(Error::ConfirmingTransactionFailed)?;
        }

        Options::RecentBlockHash { net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            let recent_hash = rpc_client
                .get_latest_blockhash()
                .map_err(Error::RecentHashFailed)?;
            println!("Recent blockhash: {}", recent_hash);
        }

        Options::AggregateKeys { keys } => {
            let aggkey = tss::key_agg(keys, None)?;
            let agg_bytes = aggkey.agg_public_key.to_bytes(true);
            let mut pubkey_bytes = [0u8; 32];
            pubkey_bytes.copy_from_slice(&agg_bytes);
            let aggpubkey = Pubkey::from(pubkey_bytes);
            println!("The Aggregated Public Key: {}", aggpubkey);
        }

        Options::CreateToken { mint_authority, generate_mint_authority, mint_authority_key, freeze_authority, decimals, initial_supply, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            // Get mint authority keypair - from file, generate new one, or use provided key
            let mint_authority_keypair = if generate_mint_authority {
                let keypair = Keypair::new();
                println!("Generated new mint authority:");
                println!("Private key (base58): {}", keypair.to_base58_string());
                println!("Public key: {}", keypair.pubkey());
                println!();
                keypair
            } else if let Some(mint_auth_file) = mint_authority {
                read_keypair_file(&mint_auth_file)?
            } else if let Some(private_key) = mint_authority_key {
                println!("WARNING: Passing private keys via command line is less secure!");
                let decoded = bs58::decode(private_key.trim())
                    .into_vec()
                    .map_err(Error::BadBase58)?;
                Keypair::from_bytes(&decoded)
                    .map_err(Error::WrongKeyPair)?
            } else {
                return Err(Error::FileReadError("One of --mint-authority, --generate-mint-authority, or --mint-authority-key must be specified".to_string()));
            };
            
            // Read freeze authority keypair if provided
            let freeze_authority_pubkey = if let Some(freeze_auth_file) = freeze_authority {
                let freeze_keypair = read_keypair_file(&freeze_auth_file)?;
                Some(freeze_keypair.pubkey())
            } else {
                None
            };
            
            // Create the token mint
            let (mint_pubkey, signature) = token::create_token_mint(
                &rpc_client,
                &mint_authority_keypair,
                &mint_authority_keypair.pubkey(),
                freeze_authority_pubkey.as_ref(),
                decimals,
            )?;
            
            println!("Token mint created successfully!");
            println!("Mint address: {}", mint_pubkey);
            println!("Transaction signature: {}", signature);
            
            // Mint initial supply if specified (disabled - use mint-tokens command instead)
            if initial_supply > 0 {
                println!("Note: Initial supply minting is disabled. Use the 'mint-tokens' command instead:");
                println!("cargo run -- mint-tokens --mint {} --mint-authority-key <KEY> --to {} --amount {} --decimals {}", 
                         mint_pubkey, mint_authority_keypair.pubkey(), initial_supply, decimals);
            }
        }

        Options::TransferTokens { mint, from, from_key, to, amount, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            // Get sender keypair - either from file or directly from private key
            let from_keypair = if let Some(from_file) = from {
                read_keypair_file(&from_file)?
            } else if let Some(private_key) = from_key {
                println!("WARNING: Passing private keys via command line is less secure!");
                let decoded = bs58::decode(private_key.trim())
                    .into_vec()
                    .map_err(Error::BadBase58)?;
                Keypair::from_bytes(&decoded)
                    .map_err(Error::WrongKeyPair)?
            } else {
                return Err(Error::FileReadError("Either --from or --from-key must be specified".to_string()));
            };
            
            // Transfer tokens
            let signature = token::transfer_tokens(
                &rpc_client,
                &from_keypair,
                &mint,
                &from_keypair,
                &to,
                amount,
            )?;
            
            println!("Token transfer successful!");
            println!("From: {}", from_keypair.pubkey());
            println!("To: {}", to);
            println!("Amount: {} tokens", amount);
            println!("Transaction signature: {}", signature);
        }

        Options::TokenBalance { mint, wallet, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            let balance = token::get_token_balance(&rpc_client, &wallet, &mint)?;
            println!("Token balance for wallet {}: {} tokens", wallet, balance);
        }

        Options::MintTokens { mint, mint_authority, mint_authority_key, to, amount, decimals, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            // Get mint authority keypair - either from file or from private key
            let mint_authority_keypair = if let Some(mint_auth_file) = mint_authority {
                read_keypair_file(&mint_auth_file)?
            } else if let Some(private_key) = mint_authority_key {
                println!("WARNING: Passing private keys via command line is less secure!");
                let decoded = bs58::decode(private_key.trim())
                    .into_vec()
                    .map_err(Error::BadBase58)?;
                Keypair::from_bytes(&decoded)
                    .map_err(Error::WrongKeyPair)?
            } else {
                return Err(Error::FileReadError("Either --mint-authority or --mint-authority-key must be specified".to_string()));
            };
            
            // Mint tokens to the specified wallet
            let signature = token::mint_tokens_to(
                &rpc_client,
                &mint_authority_keypair,  // payer (same as mint authority for simplicity)
                &mint,
                &to,
                &mint_authority_keypair,
                amount,
                decimals,
            )?;
            
            println!("Tokens minted successfully!");
            println!("Mint: {}", mint);
            println!("To: {}", to);
            println!("Amount: {} tokens", amount);
            println!("Transaction signature: {}", signature);
        }

        Options::AggSendStepOne { private_key } => {
            println!("WARNING: Passing private keys via command line is less secure!");
            
            // Parse the private key
            let decoded = bs58::decode(private_key.trim())
                .into_vec()
                .map_err(Error::BadBase58)?;
            let keypair = Keypair::from_bytes(&decoded)
                .map_err(Error::WrongKeyPair)?;
            
            // Generate nonces for MPC step 1
            let (public_msg, secret_state) = tss::step_one(keypair);
            
            // Output the results
            println!("secret share: {}", secret_state.serialize_bs58());
            println!("public share: {}", public_msg.serialize_bs58());
        }

        Options::AggSendStepTwoToken { 
            private_key, 
            mint, 
            amount, 
            decimals, 
            to, 
            recent_block_hash, 
            keys, 
            first_messages, 
            secret_state, 
            net 
        } => {
            println!("WARNING: Passing private keys via command line is less secure!");
            
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            // Parse the private key
            let decoded = bs58::decode(private_key.trim())
                .into_vec()
                .map_err(Error::BadBase58)?;
            let keypair = Keypair::from_bytes(&decoded)
                .map_err(Error::WrongKeyPair)?;
            
            // Parse recent block hash
            let block_hash = recent_block_hash.parse::<Hash>()
                .map_err(|e| Error::FileReadError(format!("Invalid block hash: {}", e)))?;
            
            // Parse first messages
            let parsed_first_messages: Result<Vec<serialization::AggMessage1>, Error> = first_messages
                .iter()
                .map(|msg| serialization::AggMessage1::deserialize_bs58(msg))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Error::FileReadError(format!("Failed to parse first messages: {}", e)));
            let parsed_first_messages = parsed_first_messages?;
            
            // Parse secret state
            let parsed_secret_state = serialization::SecretAggStepOne::deserialize_bs58(&secret_state)
                .map_err(|e| Error::FileReadError(format!("Failed to parse secret state: {}", e)))?;
            
            // Generate partial signature for token transfer
            let partial_signature = tss::step_two_token(
                keypair,
                mint,
                amount,
                decimals,
                to,
                block_hash,
                keys,
                parsed_first_messages,
                parsed_secret_state,
                &rpc_client,
            )?;
            
            // Output the partial signature
            println!("partial signature: {}", partial_signature.serialize_bs58());
        }

        Options::AggregateSignaturesAndBroadcastToken {
            signatures,
            mint,
            amount,
            decimals,
            to,
            recent_block_hash,
            keys,
            net,
        } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            // Parse recent block hash
            let block_hash = recent_block_hash.parse::<Hash>()
                .map_err(|e| Error::FileReadError(format!("Invalid block hash: {}", e)))?;
            
            // Parse partial signatures
            let parsed_signatures: Result<Vec<serialization::PartialSignature>, Error> = signatures
                .iter()
                .map(|sig| serialization::PartialSignature::deserialize_bs58(sig))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Error::FileReadError(format!("Failed to parse signatures: {}", e)));
            let parsed_signatures = parsed_signatures?;
            
            // Aggregate signatures and create final transaction
            let tx = tss::sign_and_broadcast_token(
                mint,
                amount,
                decimals,
                to,
                block_hash,
                keys,
                parsed_signatures,
                &rpc_client,
            )?;
            
            // Send the transaction
            let signature = rpc_client.send_transaction(&tx)
                .map_err(|e| Error::FileReadError(format!("Failed to send transaction: {}", e)))?;
            
            println!("Token transfer successful!");
            println!("Transaction ID: {}", signature);
            println!("Mint: {}", mint);
            println!("To: {}", to);
            println!("Amount: {} tokens", amount);
        }

    }

    Ok(())
}
