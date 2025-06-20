use std::fmt::{Display, Formatter};

use curv::elliptic::curves::{DeserializationError, PointFromBytesError};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use multi_party_eddsa::protocols::musig2::{PrivatePartialNonces, PublicPartialNonces};
use curv::elliptic::curves::{Point, Scalar};

/// Serialization-specific error types
#[derive(Debug)]
pub enum Error {
    InputTooShort { expected: usize, found: usize },
    BadBase58(bs58::decode::Error),
    InvalidPoint(PointFromBytesError),
    InvalidScalar(DeserializationError),
    WrongTag { expected: Tag, found: Tag },
}

/// Message tags for different types of serialized data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag {
    AggregatedKey = 0,
    AggMessage1 = 1,
    PartialSignature = 2,
    SecretAggStepOne = 3,
    KeyAggMessage = 4,
    Unknown,
}

impl From<u8> for Tag {
    fn from(t: u8) -> Self {
        match t {
            _ if t == Tag::AggregatedKey as u8 => Tag::AggregatedKey,
            _ if t == Tag::AggMessage1 as u8 => Tag::AggMessage1,
            _ if t == Tag::PartialSignature as u8 => Tag::PartialSignature,
            _ if t == Tag::SecretAggStepOne as u8 => Tag::SecretAggStepOne,
            _ if t == Tag::KeyAggMessage as u8 => Tag::KeyAggMessage,
            _ => Tag::Unknown,
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::AggregatedKey => f.write_str("Aggregated Key"),
            Tag::AggMessage1 => f.write_str("Aggregation Message 1"),
            Tag::PartialSignature => f.write_str("Partial Signature"),
            Tag::SecretAggStepOne => f.write_str("Secret Aggregation Step One"),
            Tag::KeyAggMessage => f.write_str("Key Aggregation Message"),
            Tag::Unknown => f.write_str("Unknown"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputTooShort { expected, found } => {
                write!(f, "Input too short, expected: {}, found: {}", expected, found)
            }
            Self::BadBase58(e) => write!(f, "Invalid base58: {}", e),
            Self::InvalidPoint(e) => write!(f, "Invalid Ed25519 Point: {}", e),
            Self::InvalidScalar(e) => write!(f, "Invalid Ed25519 Scalar: {}", e),
            Self::WrongTag { expected, found } => {
                write!(f, "Expected to find message: {}, instead found: {}", expected, found)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<PointFromBytesError> for Error {
    fn from(e: PointFromBytesError) -> Self {
        Self::InvalidPoint(e)
    }
}

impl From<DeserializationError> for Error {
    fn from(e: DeserializationError) -> Self {
        Self::InvalidScalar(e)
    }
}

/// Trait for serializing and deserializing MPC data structures
pub trait Serialize: Sized {
    /// Serialize to base58 string
    fn serialize_bs58(&self) -> String {
        let mut vec = Vec::with_capacity(self.size_hint());
        self.serialize(&mut vec);
        bs58::encode(vec).into_string()
    }
    
    /// Serialize to bytes
    fn serialize(&self, append_to: &mut Vec<u8>);
    
    /// Deserialize from base58 string
    fn deserialize_bs58(s: impl AsRef<[u8]>) -> Result<Self, Error> {
        let out = bs58::decode(s).into_vec().map_err(Error::BadBase58)?;
        Self::deserialize(&out)
    }
    
    /// Deserialize from bytes
    fn deserialize(b: &[u8]) -> Result<Self, Error>;
    
    /// Size hint for efficient memory allocation
    fn size_hint(&self) -> usize;
}

/// Aggregated key information for serialization
#[derive(Debug, PartialEq)]
pub struct AggregatedKeyInfo {
    pub aggregated_pubkey: Pubkey,
    pub participant_keys: Vec<Pubkey>,
}

impl Serialize for AggregatedKeyInfo {
    fn serialize(&self, append_to: &mut Vec<u8>) {
        append_to.reserve(self.size_hint());
        append_to.push(Tag::AggregatedKey as u8);
        
        // Serialize aggregated pubkey
        append_to.extend(self.aggregated_pubkey.to_bytes());
        
        // Serialize number of participant keys
        append_to.extend((self.participant_keys.len() as u32).to_le_bytes());
        
        // Serialize each participant key
        for key in &self.participant_keys {
            append_to.extend(key.to_bytes());
        }
    }
    
    fn deserialize(b: &[u8]) -> Result<Self, Error> {
        if b.len() < 1 + 32 + 4 {
            return Err(Error::InputTooShort { expected: 1 + 32 + 4, found: b.len() });
        }
        
        let tag = Tag::from(b[0]);
        if tag != Tag::AggregatedKey {
            return Err(Error::WrongTag { expected: Tag::AggregatedKey, found: tag });
        }
        
        // Deserialize aggregated pubkey
        let mut agg_bytes = [0u8; 32];
        agg_bytes.copy_from_slice(&b[1..33]);
        let aggregated_pubkey = Pubkey::from(agg_bytes);
        
        // Deserialize number of participant keys
        let num_keys = u32::from_le_bytes([b[33], b[34], b[35], b[36]]) as usize;
        
        // Check if we have enough data for all keys
        let expected_len = 1 + 32 + 4 + (num_keys * 32);
        if b.len() < expected_len {
            return Err(Error::InputTooShort { expected: expected_len, found: b.len() });
        }
        
        // Deserialize participant keys
        let mut participant_keys = Vec::with_capacity(num_keys);
        for i in 0..num_keys {
            let start = 37 + (i * 32);
            let end = start + 32;
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(&b[start..end]);
            participant_keys.push(Pubkey::from(key_bytes));
        }
        
        Ok(Self {
            aggregated_pubkey,
            participant_keys,
        })
    }
    
    fn size_hint(&self) -> usize {
        1 + 32 + 4 + (self.participant_keys.len() * 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::{Keypair, Signer};
    
    #[test]
    fn test_aggregated_key_info_serialization() {
        let mut rng = rand07::thread_rng();
        
        // Generate test data
        let agg_keypair = Keypair::generate(&mut rng);
        let participant_keypairs: Vec<Keypair> = (0..3).map(|_| Keypair::generate(&mut rng)).collect();
        let participant_keys: Vec<Pubkey> = participant_keypairs.iter().map(|k| k.pubkey()).collect();
        
        let agg_info = AggregatedKeyInfo {
            aggregated_pubkey: agg_keypair.pubkey(),
            participant_keys: participant_keys.clone(),
        };
        
        // Test binary serialization
        let mut serialized = Vec::new();
        agg_info.serialize(&mut serialized);
        let deserialized = AggregatedKeyInfo::deserialize(&serialized).unwrap();
        
        assert_eq!(agg_info, deserialized);
        
        // Test base58 serialization
        let base58_str = agg_info.serialize_bs58();
        let deserialized_b58 = AggregatedKeyInfo::deserialize_bs58(&base58_str).unwrap();
        
        assert_eq!(agg_info, deserialized_b58);
    }
}

/// Message containing public nonces for MPC nonce generation (step 1)
#[derive(Debug, PartialEq)]
pub struct AggMessage1 {
    pub public_nonces: PublicPartialNonces,
    pub sender: Pubkey,
}

impl Serialize for AggMessage1 {
    fn serialize(&self, append_to: &mut Vec<u8>) {
        append_to.reserve(self.size_hint());
        append_to.push(Tag::AggMessage1 as u8);
        append_to.extend(&*self.public_nonces.R[0].to_bytes(true));
        append_to.extend(&*self.public_nonces.R[1].to_bytes(true));
        append_to.extend(self.sender.to_bytes());
    }
    
    fn deserialize(b: &[u8]) -> Result<Self, Error> {
        if b.len() < 1 + 32 + 32 + 32 {
            return Err(Error::InputTooShort { expected: 1 + 32 + 32 + 32, found: b.len() });
        }
        let tag = Tag::from(b[0]);
        if tag != Tag::AggMessage1 {
            return Err(Error::WrongTag { expected: Tag::AggMessage1, found: tag });
        }
        let public_nonces =
            PublicPartialNonces { R: [Point::from_bytes(&b[1..32 + 1])?, Point::from_bytes(&b[1 + 32..64 + 1])?] };
        let mut sender_bytes = [0u8; 32];
        sender_bytes.copy_from_slice(&b[64 + 1..64 + 32 + 1]);
        let sender = Pubkey::from(sender_bytes);
        Ok(Self { public_nonces, sender })
    }
    
    fn size_hint(&self) -> usize {
        1 + 32 + 32 + 32
    }
}

/// Secret state from step one of MPC nonce generation
#[derive(Debug, PartialEq)]
pub struct SecretAggStepOne {
    pub private_nonces: PrivatePartialNonces,
    pub public_nonces: PublicPartialNonces,
}

impl Serialize for SecretAggStepOne {
    fn serialize(&self, append_to: &mut Vec<u8>) {
        append_to.reserve(self.size_hint());
        append_to.push(Tag::SecretAggStepOne as u8);

        append_to.extend(&*self.private_nonces.r[0].to_bytes());
        append_to.extend(&*self.private_nonces.r[1].to_bytes());
        append_to.extend(&*self.public_nonces.R[0].to_bytes(true));
        append_to.extend(&*self.public_nonces.R[1].to_bytes(true));
    }
    
    fn deserialize(b: &[u8]) -> Result<Self, Error> {
        if b.len() < 1 + 32 + 32 + 32 + 32 {
            return Err(Error::InputTooShort { expected: 1 + 32 + 32 + 32 + 32, found: b.len() });
        }

        let tag = Tag::from(b[0]);
        if tag != Tag::SecretAggStepOne {
            return Err(Error::WrongTag { expected: Tag::SecretAggStepOne, found: tag });
        }
        let private_nonces =
            PrivatePartialNonces { r: [Scalar::from_bytes(&b[1..1 + 32])?, Scalar::from_bytes(&b[1 + 32..1 + 64])?] };
        #[allow(non_snake_case)]
        let public_nonces = PublicPartialNonces {
            R: [Point::from_bytes(&b[1 + 64..1 + 64 + 32])?, Point::from_bytes(&b[1 + 96..1 + 96 + 32])?],
        };
        Ok(Self { private_nonces, public_nonces })
    }
    
    fn size_hint(&self) -> usize {
        1 + 32 + 32 + 32 + 32
    }
}

/// Partial signature for MPC signing
#[derive(Debug, PartialEq)]
pub struct PartialSignature(pub Signature);

impl Serialize for PartialSignature {
    fn serialize(&self, append_to: &mut Vec<u8>) {
        append_to.reserve(self.size_hint());
        append_to.push(Tag::PartialSignature as u8);
        append_to.extend(self.0.as_ref());
    }
    
    fn deserialize(b: &[u8]) -> Result<Self, Error> {
        if b.len() < 1 + 64 {
            return Err(Error::InputTooShort { expected: 1 + 64, found: b.len() });
        }
        let tag = Tag::from(b[0]);
        if tag != Tag::PartialSignature {
            return Err(Error::WrongTag { expected: Tag::PartialSignature, found: tag });
        }
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(&b[1..1 + 64]);
        Ok(PartialSignature(Signature::from(sig_bytes)))
    }
    
    fn size_hint(&self) -> usize {
        1 + 64
    }
}
