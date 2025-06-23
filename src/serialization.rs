use std::fmt::{Display, Formatter};

use curv::elliptic::curves::{DeserializationError, Ed25519, Point, PointFromBytesError, Scalar};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use multi_party_eddsa::protocols::musig2::{self, PrivatePartialNonces, PublicPartialNonces, PartialSignature as Musig2PartialSignature};

/// Serialization-specific error types
#[derive(Debug)]
pub enum Error {
    InputTooShort { expected: usize, found: usize },
    BadBase58(bs58::decode::Error),
    InvalidPoint(PointFromBytesError),
    InvalidScalar(DeserializationError),
    WrongTag { expected: String, found: String },
    PointDeserializationFailed { field_name: &'static str },
    ScalarDeserializationFailed { field_name: &'static str },
    MismatchMessages,
}

/// Message tags for different types of serialized data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag {
    AggMessage1 = 1,
    PartialSignature = 2,
    SecretAggStepOne = 3,
}

impl From<u8> for Tag {
    fn from(t: u8) -> Self {
        match t {
            _ if t == Tag::AggMessage1 as u8 => Tag::AggMessage1,
            _ if t == Tag::PartialSignature as u8 => Tag::PartialSignature,
            _ if t == Tag::SecretAggStepOne as u8 => Tag::SecretAggStepOne,
            _ => panic!("Unknown tag: {}", t),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::AggMessage1 => f.write_str("Aggregation Message 1"),
            Tag::PartialSignature => f.write_str("Partial Signature"),
            Tag::SecretAggStepOne => f.write_str("Secret Aggregation Step One"),
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
            Self::PointDeserializationFailed { field_name } => {
                write!(f, "Failed to deserialize point from bytes for field: {}", field_name)
            }
            Self::ScalarDeserializationFailed { field_name } => {
                write!(f, "Failed to deserialize scalar from bytes for field: {}", field_name)
            }
            Self::MismatchMessages => write!(f, "Mismatch in messages"),
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
            return Err(Error::WrongTag { expected: Tag::AggMessage1.to_string(), found: tag.to_string() });
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
            return Err(Error::WrongTag { expected: Tag::SecretAggStepOne.to_string(), found: tag.to_string() });
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
            return Err(Error::WrongTag { expected: Tag::PartialSignature.to_string(), found: tag.to_string() });
        }
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(&b[1..1 + 64]);
        Ok(PartialSignature(Signature::from(sig_bytes)))
    }
    
    fn size_hint(&self) -> usize {
        1 + 64
    }
}

impl PartialSignature {
    pub fn deserialize_r(s: &[u8]) -> Result<Point<Ed25519>, Error> {
        Point::from_bytes(s).map_err(|_e| Error::PointDeserializationFailed { field_name: "signatures R component" })
    }

    pub fn deserialize_s(s: &[u8]) -> Result<Scalar<Ed25519>, Error> {
        Scalar::from_bytes(s).map_err(|_e| Error::ScalarDeserializationFailed { field_name: "signatures s component" })
    }

    pub fn to_musig2_partial_signature(&self) -> Result<Musig2PartialSignature, Error> {
        Ok(Musig2PartialSignature {
            R: Self::deserialize_r(&self.0.as_ref()[..32])?,
            my_partial_s: Self::deserialize_s(&self.0.as_ref()[32..])?,
        })
    }

    pub fn aggregate_signatures(signatures: &[PartialSignature]) -> Result<Signature, Error> {
        if signatures.is_empty() {
            return Err(Error::InputTooShort { expected: 1, found: 0 });
        }

        // Make sure all the `R`s are the same
        if !signatures[1..].iter()
            .map(|s| &s.0.as_ref()[..32])
            .all(|s| s == &signatures[0].0.as_ref()[..32]) {
            return Err(Error::MismatchMessages);
        }

        // Convert first signature to MuSig2 format
        let first_sig = signatures[0].to_musig2_partial_signature()?;

        // Convert remaining signatures
        let partial_sigs: Vec<_> = signatures[1..]
            .iter()
            .map(|s| Self::deserialize_s(&s.0.as_ref()[32..]))
            .collect::<Result<_, _>>()?;

        // Aggregate using MuSig2
        let full_sig = musig2::aggregate_partial_signatures(&first_sig, &partial_sigs);

        // Convert to Solana format
        let mut sig_bytes = [0u8; 64];
        sig_bytes[..32].copy_from_slice(&*full_sig.R.to_bytes(true));
        sig_bytes[32..].copy_from_slice(&full_sig.s.to_bytes());
        
        Ok(Signature::from(sig_bytes))
    }
}
