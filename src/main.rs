use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token, signature::{Signer, Keypair}, hash::Hash};

mod cli;
mod error;
mod tss;
mod serialization;
mod token;

use cli::{Options};
use error::Error;
use serialization::Serialize;

// Helper function to parse a private key from base58 string
fn parse_keypair(private_key: &str) -> Result<Keypair, Error> {
    let decoded = bs58::decode(private_key.trim())
        .into_vec()
        .map_err(Error::BadBase58)?;
    Keypair::from_bytes(&decoded)
        .map_err(Error::WrongKeyPair)
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
            let aggpubkey = tss::agg_key_to_pubkey(&aggkey);
            println!("The Aggregated Public Key: {}", aggpubkey);
        }

        Options::CreateToken { mint_authority_key, freeze_authority_key, decimals, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            
            // Handle mint authority - either generate new or parse provided key
            let mint_authority_keypair = if mint_authority_key == "generate" {
                let keypair = Keypair::new();
                println!("Generated new mint authority:");
                println!("Private key (base58): {}", keypair.to_base58_string());
                println!("Public key: {}", keypair.pubkey());
                println!();
                keypair
            } else {
                parse_keypair(&mint_authority_key)?
            };
            
            // Parse freeze authority if provided
            let freeze_authority_pubkey = if let Some(freeze_key) = freeze_authority_key {
                let freeze_keypair = parse_keypair(&freeze_key)?;
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
        }

        Options::TransferTokens { mint, from_key, to, amount, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            let from_keypair = parse_keypair(&from_key)?;
            
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

        Options::MintTokens { mint, mint_authority_key, to, amount, decimals, net } => {
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            let mint_authority_keypair = parse_keypair(&mint_authority_key)?;
            
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
            let keypair = parse_keypair(&private_key)?;
            
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
            let rpc_client = RpcClient::new(net.get_cluster_url().to_string());
            let keypair = parse_keypair(&private_key)?;
            
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
