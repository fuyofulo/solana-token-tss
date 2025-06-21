use std::str::FromStr;

use clap::{Parser, ValueEnum};
use solana_sdk::{pubkey::Pubkey};

use crate::error::Error;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub enum Options {
    /// Generate a pair of keys.
    #[clap(display_order = 1)]
    Generate,

    /// Check the balance of an address.
    #[clap(display_order = 2)]
    Balance {
        /// The address to check the balance of
        address: Pubkey,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "testnet", long)]
        net: Network,
    },

    /// Request an airdrop from a faucet.
    #[clap(display_order = 3)]
    Airdrop {
        /// Address of the recipient
        #[clap(long)]
        to: Pubkey,
        /// The amount of SOL you want to send.
        #[clap(long)]
        amount: f64,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "testnet", long)]
        net: Network,
    },

    /// Fetch and print the recent blockhash.
    #[clap(display_order = 4)]
    RecentBlockHash {
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "testnet", long)]
        net: Network,
    },

    /// Aggregate a list of addresses into a single address that they can all sign on together
    #[clap(display_order = 5)]
    AggregateKeys {
        /// List of addresses
        #[clap(min_values = 2, required = true)]
        keys: Vec<solana_sdk::pubkey::Pubkey>,
    },

    /// Create a new SPL token mint
    #[clap(display_order = 6)]
    CreateToken {
        /// Private key (base58) for mint authority (who can mint tokens). Use 'generate' to auto-generate.
        #[clap(long)]
        mint_authority_key: String,
        /// Private key (base58) for freeze authority (optional - who can freeze accounts)
        #[clap(long)]
        freeze_authority_key: Option<String>,
        /// Number of decimal places for the token (0-9)
        #[clap(long, default_value = "6")]
        decimals: u8,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Transfer tokens from one wallet to another
    #[clap(display_order = 7)]
    TransferTokens {
        /// Token mint address
        #[clap(long)]
        mint: Pubkey,
        /// Private key (base58) for sender wallet
        #[clap(long)]
        from_key: String,
        /// Public key of the recipient wallet
        #[clap(long)]
        to: Pubkey,
        /// Amount of tokens to transfer (in smallest unit)
        #[clap(long)]
        amount: u64,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Check token balance for a wallet
    #[clap(display_order = 8)]
    TokenBalance {
        /// Token mint address
        #[clap(long)]
        mint: Pubkey,
        /// Wallet public key to check balance for
        #[clap(long)]
        wallet: Pubkey,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Mint tokens to a wallet
    #[clap(display_order = 9)]
    MintTokens {
        /// Token mint address
        #[clap(long)]
        mint: Pubkey,
        /// Private key (base58) for mint authority
        #[clap(long)]
        mint_authority_key: String,
        /// Public key of the recipient wallet
        #[clap(long)]
        to: Pubkey,
        /// Amount of tokens to mint (in smallest unit)
        #[clap(long)]
        amount: u64,
        /// Number of decimal places for the token
        #[clap(long, default_value = "6")]
        decimals: u8,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Generate nonces for MPC token transfer (Step 1)
    #[clap(display_order = 10)]
    AggSendStepOne {
        /// Private key (base58) of the party participating in MPC signing
        private_key: String,
    },

    /// Generate partial signature for MPC token transfer (Step 2)
    #[clap(display_order = 11)]
    AggSendStepTwoToken {
        /// Private key (base58) of the party participating in MPC signing
        #[clap(long)]
        private_key: String,
        /// Token mint address
        #[clap(long)]
        mint: Pubkey,
        /// Amount of tokens to transfer (in smallest unit)
        #[clap(long)]
        amount: u64,
        /// Number of decimal places for the token
        #[clap(long)]
        decimals: u8,
        /// Public key of the recipient wallet
        #[clap(long)]
        to: Pubkey,
        /// Recent block hash (all parties must use the same hash)
        #[clap(long)]
        recent_block_hash: String,
        /// List of all participant public keys (comma-separated)
        #[clap(long, value_delimiter = ',')]
        keys: Vec<Pubkey>,
        /// List of first messages from step 1 (comma-separated base58 strings)
        #[clap(long, value_delimiter = ',')]
        first_messages: Vec<String>,
        /// Secret state from step 1 (base58 string)
        #[clap(long)]
        secret_state: String,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Aggregate partial signatures and broadcast token transfer transaction (Step 3)
    #[clap(display_order = 12)]
    AggregateSignaturesAndBroadcastToken {
        /// List of all partial signatures from step 2 (comma-separated base58 strings)
        #[clap(long, value_delimiter = ',')]
        signatures: Vec<String>,
        /// Token mint address
        #[clap(long)]
        mint: Pubkey,
        /// Amount of tokens to transfer (in smallest unit)
        #[clap(long)]
        amount: u64,
        /// Number of decimal places for the token
        #[clap(long)]
        decimals: u8,
        /// Public key of the recipient wallet
        #[clap(long)]
        to: Pubkey,
        /// Recent block hash (all parties must use the same hash)
        #[clap(long)]
        recent_block_hash: String,
        /// List of all participant public keys (comma-separated)
        #[clap(long, value_delimiter = ',')]
        keys: Vec<Pubkey>,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Generate partial signature for MPC SOL transfer (Step 2)
    #[clap(display_order = 13)]
    AggSendStepTwoSol {
        /// Private key (base58) of the party participating in MPC signing
        #[clap(long)]
        private_key: String,
        /// Amount of SOL to transfer
        #[clap(long)]
        amount: f64,
        /// Public key of the recipient wallet
        #[clap(long)]
        to: Pubkey,
        /// Optional memo to attach to the transaction
        #[clap(long)]
        memo: Option<String>,
        /// Recent block hash (all parties must use the same hash)
        #[clap(long)]
        recent_block_hash: String,
        /// List of all participant public keys (comma-separated)
        #[clap(long, value_delimiter = ',')]
        keys: Vec<Pubkey>,
        /// List of first messages from step 1 (comma-separated base58 strings)
        #[clap(long, value_delimiter = ',')]
        first_messages: Vec<String>,
        /// Secret state from step 1 (base58 string)
        #[clap(long)]
        secret_state: String,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },

    /// Aggregate partial signatures and broadcast SOL transfer transaction (Step 3)
    #[clap(display_order = 14)]
    AggregateSignaturesAndBroadcastSol {
        /// List of all partial signatures from step 2 (comma-separated base58 strings)
        #[clap(long, value_delimiter = ',')]
        signatures: Vec<String>,
        /// Amount of SOL to transfer
        #[clap(long)]
        amount: f64,
        /// Public key of the recipient wallet
        #[clap(long)]
        to: Pubkey,
        /// Optional memo to attach to the transaction
        #[clap(long)]
        memo: Option<String>,
        /// Recent block hash (all parties must use the same hash)
        #[clap(long)]
        recent_block_hash: String,
        /// List of all participant public keys (comma-separated)
        #[clap(long, value_delimiter = ',')]
        keys: Vec<Pubkey>,
        /// Choose the desired network: Mainnet/Testnet/Devnet/Localnet
        #[clap(default_value = "localnet", long)]
        net: Network,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Network {
    Mainnet,
    Testnet,
    Devnet,
    Localnet,
}

impl Network {
    pub fn get_cluster_url(&self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.mainnet-beta.solana.com",
            Self::Testnet => "https://api.testnet.solana.com",
            Self::Devnet => "https://api.devnet.solana.com",
            Self::Localnet => "http://127.0.0.1:8899",
        }
    }
}

impl FromStr for Network {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(Self::Mainnet),
            "testnet" => Ok(Self::Testnet),
            "devnet" => Ok(Self::Devnet),
            "localnet" | "local" => Ok(Self::Localnet),
            _ => Err(Error::WrongNetwork(s.to_string())),
        }
    }
}
