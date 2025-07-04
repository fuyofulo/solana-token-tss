use std::fmt::{Display, Formatter};

use bs58::decode::Error as Bs58Error;
use ed25519_dalek::SignatureError;
use solana_client::client_error::ClientError;
use crate::serialization;

/// Custom application error type
#[derive(Debug)]
pub enum Error {
    WrongNetwork(String),
    BadBase58(Bs58Error),
    WrongKeyPair(SignatureError),
    AirdropFailed(ClientError),
    RecentHashFailed(ClientError),
    ConfirmingTransactionFailed(ClientError),
    BalaceFailed(ClientError),
    KeyPairIsNotInKeys,
    InvalidSignature,
    TokenCreationFailed(String),
    TokenMintFailed(String),
    TokenTransferFailed(String),
    TokenAccountNotFound,
    FileReadError(String),
    SerializationError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongNetwork(net) => write!(
                f,
                "Unrecognized network: {}, please select Mainnet/Testnet/Devnet/Localnet",
                net
            ),
            Self::BadBase58(e) => write!(f, "Base58 decoding error: {}", e),
            Self::WrongKeyPair(e) => write!(f, "Failed to deserialize keypair: {}", e),
            Self::AirdropFailed(e) => write!(f, "Airdrop failed: {}", e),
            Self::RecentHashFailed(e) => write!(f, "Failed to get recent blockhash: {}", e),
            Self::ConfirmingTransactionFailed(e) => write!(f, "Transaction confirmation failed: {}", e),
            Self::BalaceFailed(e) => write!(f, "Balance query failed: {}", e),
            Self::KeyPairIsNotInKeys => write!(f, "The provided keypair is not in the list of pubkeys"),
            Self::InvalidSignature => write!(f, "Invalid signature"),
            Self::TokenCreationFailed(e) => write!(f, "Token creation failed: {}", e),
            Self::TokenMintFailed(e) => write!(f, "Token minting failed: {}", e),
            Self::TokenTransferFailed(e) => write!(f, "Token transfer failed: {}", e),
            Self::TokenAccountNotFound => write!(f, "Token account not found"),
            Self::FileReadError(e) => write!(f, "File read error: {}", e),
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<Bs58Error> for Error {
    fn from(e: Bs58Error) -> Self {
        Self::BadBase58(e)
    }
}

impl From<SignatureError> for Error {
    fn from(e: SignatureError) -> Self {
        Self::WrongKeyPair(e)
    }
}

impl From<serialization::Error> for Error {
    fn from(e: serialization::Error) -> Self {
        Error::SerializationError(e.to_string())
    }
}

impl std::error::Error for Error {}
