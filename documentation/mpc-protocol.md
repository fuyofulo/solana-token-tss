# MPC Protocol Guide

This document provides an in-depth explanation of the Multi-Party Computation (MPC) protocol used in Solana MPC Tokens, specifically the **MuSig2** implementation for multi-party signatures.

## 🔐 What is MuSig2?

**MuSig2** is a state-of-the-art **multi-signature scheme** that allows multiple parties to collaboratively create signatures that appear as single signatures on-chain. Unlike traditional multi-signature schemes, MuSig2 signatures are:

- **Indistinguishable** from single-party signatures
- **Constant size** regardless of the number of signers
- **Efficient** in both computation and bandwidth
- **Secure** against various cryptographic attacks

## 🎯 Key Advantages

| Feature | Traditional Multi-Sig | MuSig2 |
|---------|----------------------|--------|
| **On-chain footprint** | Large (per signer) | Constant (single signature) |
| **Privacy** | Reveals number of signers | Looks like single signature |
| **Gas costs** | High (per signer) | Low (single signature) |
| **Verification** | O(n) complexity | O(1) complexity |

## 🔄 Protocol Overview

The MuSig2 protocol consists of **three main phases**:

```
Phase 1: Key Aggregation
├── Each party has private key xi
├── Each party computes public key Xi = xi * G
└── Aggregate: X = X1 + X2 + ... + Xn

Phase 2: Nonce Generation & Commitment
├── Each party generates random nonces ri1, ri2
├── Each party computes Ri1 = ri1 * G, Ri2 = ri2 * G
└── Parties exchange nonce commitments

Phase 3: Partial Signing & Aggregation
├── Each party creates partial signature si
├── Aggregator combines: s = s1 + s2 + ... + sn  
└── Final signature = (R, s)
```

## 🏗️ Implementation Details

### 1. Key Aggregation (`key_agg`)

**Purpose**: Combine multiple public keys into a single aggregated key.

```rust
pub fn key_agg(keys: Vec<Pubkey>, key: Option<Pubkey>) -> Result<PublicKeyAgg, Error>
```

**Process**:
1. Convert Solana `Pubkey` to Ed25519 `Point` objects
2. Compute aggregation coefficients for each key
3. Create aggregated public key: `X_agg = Σ(ai * Xi)`
4. Return `PublicKeyAgg` containing the aggregated key

**Security**: Each key gets a unique coefficient to prevent rogue key attacks.

### 2. Step One - Nonce Generation (`step_one`)

**Purpose**: Generate cryptographic nonces for the signing session.

```rust
pub fn step_one(keypair: Keypair) -> (AggMessage1, SecretAggStepOne)
```

**Process**:
1. Create `ExpandedKeyPair` from private key
2. Generate two random nonces: `r1, r2`
3. Compute public nonces: `R1 = r1 * G, R2 = r2 * G`
4. Return public message (for sharing) and secret state (keep private)

**Output**:
- **Public Message**: `{sender: Pubkey, R: [R1, R2]}` - shared with all parties
- **Secret State**: `{r: [r1, r2], R: [R1, R2]}` - kept private

### 3. Step Two - Partial Signing (`step_two_token`)

**Purpose**: Create a partial signature for the token transaction.

```rust
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
) -> Result<PartialSignature, Error>
```

**Process**:
1. Aggregate public keys and compute coefficients
2. Construct the unsigned token transaction
3. Create `PartialSigner` with nonces and aggregated key
4. Sign transaction using MuSig2 partial signing
5. Extract and return the partial signature

**Security**: Each party signs the same transaction message, ensuring consistency.

### 4. Signature Aggregation (`sign_and_broadcast_token`)

**Purpose**: Combine partial signatures into a final signature and broadcast.

```rust
pub fn sign_and_broadcast_token(
    mint: Pubkey,
    amount: u64,
    decimals: u8, 
    to: Pubkey,
    recent_block_hash: Hash,
    keys: Vec<Pubkey>,
    signatures: Vec<PartialSignature>,
    rpc_client: &RpcClient,
) -> Result<Transaction, Error>
```

**Process**:
1. Verify all partial signatures have the same R component
2. Deserialize R and s components from each partial signature
3. Aggregate partial s values: `s_final = Σ(si)`
4. Construct final signature: `(R, s_final)`
5. Create signed transaction and verify validity
6. Return transaction ready for broadcast

## 🔒 Cryptographic Security

### Elliptic Curve Operations

All operations use the **Ed25519** elliptic curve:
- **Base point**: G (generator of the curve)
- **Scalar multiplication**: `s * G` for scalar s and point G
- **Point addition**: `P1 + P2` for points P1, P2
- **Hash function**: SHA-512 for challenges and commitments

### Security Properties

1. **Unforgeability**: Cannot create valid signatures without all private keys
2. **Non-malleability**: Signatures cannot be modified without detection
3. **Provable Security**: Based on discrete logarithm problem
4. **Perfect Privacy**: Individual keys remain completely private

### Threat Model

**Protected Against**:
- ✅ **Rogue Key Attacks**: Coefficients prevent malicious key selection
- ✅ **Wagner's Attack**: Two nonces per party prevent k-sum attacks
- ✅ **Parallel Session Attacks**: Fresh nonces for each session
- ✅ **Key Substitution**: Coefficient binding prevents substitution

**Requires Trust For**:
- 🔒 **Honest Majority**: All parties must participate honestly (n-of-n)
- 🔒 **Secure Channels**: Off-chain communication should be authenticated
- 🔒 **Nonce Freshness**: Parties must use fresh random nonces

## 📡 Communication Protocol

### Message Flow

```
Party 1                     Party 2                     Party 3
   │                           │                           │
   ├─── Generate Nonces ───────┤─── Generate Nonces ───────┤
   │                           │                           │
   ├────── AggMessage1 ───────▶│◀────── AggMessage1 ───────┤
   │◀───── AggMessage1 ─────────┤────── AggMessage1 ──────▶│
   │                           │                           │
   ├─── Create Partial Sig ────┤─── Create Partial Sig ────┤
   │                           │                           │
   ├──── PartialSignature ────▶│◀──── PartialSignature ────┤
   │◀──── PartialSignature ────┤──── PartialSignature ────▶│
   │                           │                           │
   └── Anyone can aggregate and broadcast final transaction ──┘
```

### Message Types

#### AggMessage1 (Step 1)
```rust
pub struct AggMessage1 {
    pub public_nonces: PublicPartialNonces,  // [R1, R2]
    pub sender: Pubkey,                      // Sender's public key
}
```

#### SecretAggStepOne (Step 1 - Private)
```rust
pub struct SecretAggStepOne {
    pub private_nonces: PrivatePartialNonces,  // [r1, r2] - KEEP SECRET
    pub public_nonces: PublicPartialNonces,    // [R1, R2] - for verification
}
```

#### PartialSignature (Step 2)
```rust
pub struct PartialSignature(pub Signature);  // 64-byte signature
```

## 🛠️ Implementation Example

### Complete 2-Party Token Transfer

```bash
# Setup: Both parties have private keys and agree on transaction details
MINT="<token_mint_address>"
AMOUNT="1000000"  # 1 token with 6 decimals
TO="<recipient_address>"
RECENT_HASH="<recent_blockhash>"

# Step 1: Both parties generate nonces
# Party 1:
cargo run -- agg-send-step-one $PRIVATE_KEY_1
# Output: secret share: <secret1_base58>
#         public share: <public1_base58>

# Party 2:
cargo run -- agg-send-step-one $PRIVATE_KEY_2  
# Output: secret share: <secret2_base58>
#         public share: <public2_base58>

# Step 2: Both parties create partial signatures
# Party 1:
cargo run -- agg-send-step-two-token \
  --private-key $PRIVATE_KEY_1 \
  --mint $MINT \
  --amount $AMOUNT \
  --decimals 6 \
  --to $TO \
  --recent-block-hash $RECENT_HASH \
  --keys $PUBLIC_KEY_1,$PUBLIC_KEY_2 \
  --first-messages $PUBLIC1_BASE58,$PUBLIC2_BASE58 \
  --secret-state $SECRET1_BASE58
# Output: partial signature: <partial_sig1_base58>

# Party 2:
cargo run -- agg-send-step-two-token \
  --private-key $PRIVATE_KEY_2 \
  --mint $MINT \
  --amount $AMOUNT \
  --decimals 6 \
  --to $TO \
  --recent-block-hash $RECENT_HASH \
  --keys $PUBLIC_KEY_1,$PUBLIC_KEY_2 \
  --first-messages $PUBLIC1_BASE58,$PUBLIC2_BASE58 \
  --secret-state $SECRET2_BASE58
# Output: partial signature: <partial_sig2_base58>

# Step 3: Anyone can aggregate and broadcast
cargo run -- aggregate-signatures-and-broadcast-token \
  --signatures $PARTIAL_SIG1_BASE58,$PARTIAL_SIG2_BASE58 \
  --mint $MINT \
  --amount $AMOUNT \
  --decimals 6 \
  --to $TO \
  --recent-block-hash $RECENT_HASH \
  --keys $PUBLIC_KEY_1,$PUBLIC_KEY_2
# Output: Token transfer successful! Transaction ID: <tx_hash>
```

## ⚠️ Security Considerations

### Critical Security Rules

1. **Never Reuse Nonces**: Each signing session must use fresh random nonces
2. **Verify All Messages**: Check that all parties use the same transaction details
3. **Secure Communication**: Use authenticated channels for message exchange
4. **Fresh Blockhashes**: Use recent blockhashes to prevent replay attacks

### Implementation Safeguards

- **R Component Verification**: All partial signatures must have identical R values
- **Transaction Verification**: Final transaction is verified before broadcast
- **Error Handling**: Protocol aborts on any cryptographic inconsistency
- **Type Safety**: Rust's type system prevents many protocol violations

## 🔬 Advanced Topics

### Performance Optimizations

- **Preprocessing**: Pre-compute aggregation coefficients
- **Batch Verification**: Verify multiple partial signatures together
- **Parallel Processing**: Generate nonces and partial signatures in parallel

---

**Next**: Explore [CLI Commands](./cli-reference.md) or dive into [Security Considerations](./security.md). 