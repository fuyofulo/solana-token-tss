# API Reference

This document provides comprehensive technical documentation for all modules, functions, and data structures in Solana MPC Tokens.

## 📚 Module Overview

- **[main.rs](#mainrs)**: Entry point and command orchestration
- **[cli.rs](#clirs)**: Command-line interface definitions
- **[tss.rs](#tssrs)**: Threshold signature scheme implementation
- **[token.rs](#tokenrs)**: SPL token operations
- **[serialization.rs](#serializationrs)**: MPC message serialization
- **[error.rs](#errorrs)**: Error handling and types

---

## main.rs

**Purpose**: Application entry point that handles command parsing and execution.

### Functions

#### `parse_keypair(private_key: &str) -> Result<Keypair, Error>`

Converts a base58-encoded private key string into a Solana `Keypair`.

**Parameters:**
- `private_key`: Base58-encoded private key string

**Returns:**
- `Ok(Keypair)`: Successfully parsed keypair
- `Err(Error)`: Invalid key format or decoding error

**Example Usage:**
```rust
let keypair = parse_keypair("5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x")?;
```

#### `main() -> Result<(), Error>`

Main application entry point that parses CLI commands and routes them to appropriate handlers.

**Process Flow:**
1. Parse command-line arguments using `clap`
2. Match on command type
3. Execute appropriate module functions
4. Handle errors and provide user feedback

---

## cli.rs

**Purpose**: Defines the command-line interface structure and argument parsing.

### Enums

#### `Options`

Main command enumeration that defines all available CLI commands.

**Variants:**

##### `Generate`
Generate a new keypair.
```rust
Generate
```

##### `Balance`
Check SOL balance of an address.
```rust
Balance {
    address: Pubkey,
    #[clap(default_value = "testnet", long)]
    net: Network,
}
```

##### `Airdrop`
Request SOL from a faucet.
```rust
Airdrop {
    #[clap(long)]
    to: Pubkey,
    #[clap(long)]
    amount: f64,
    #[clap(default_value = "testnet", long)]
    net: Network,
}
```

##### `RecentBlockHash`
Fetch recent blockhash.
```rust
RecentBlockHash {
    #[clap(default_value = "testnet", long)]
    net: Network,
}
```

##### `AggregateKeys`
Aggregate multiple public keys.
```rust
AggregateKeys {
    #[clap(min_values = 2, required = true)]
    keys: Vec<Pubkey>,
}
```

##### `CreateToken`
Create a new SPL token mint.
```rust
CreateToken {
    #[clap(long)]
    mint_authority_key: String,
    #[clap(long)]
    freeze_authority_key: Option<String>,
    #[clap(long, default_value = "6")]
    decimals: u8,
    #[clap(default_value = "localnet", long)]
    net: Network,
}
```

##### `TransferTokens`
Transfer tokens between wallets.
```rust
TransferTokens {
    #[clap(long)]
    mint: Pubkey,
    #[clap(long)]
    from_key: String,
    #[clap(long)]
    to: Pubkey,
    #[clap(long)]
    amount: u64,
    #[clap(default_value = "localnet", long)]
    net: Network,
}
```

##### `TokenBalance`
Check token balance.
```rust
TokenBalance {
    #[clap(long)]
    mint: Pubkey,
    #[clap(long)]
    wallet: Pubkey,
    #[clap(default_value = "localnet", long)]
    net: Network,
}
```

##### `MintTokens`
Mint tokens to a wallet.
```rust
MintTokens {
    #[clap(long)]
    mint: Pubkey,
    #[clap(long)]
    mint_authority_key: String,
    #[clap(long)]
    to: Pubkey,
    #[clap(long)]
    amount: u64,
    #[clap(long, default_value = "6")]
    decimals: u8,
    #[clap(default_value = "localnet", long)]
    net: Network,
}
```

##### `AggSendStepOne`
MPC Step 1: Generate nonces.
```rust
AggSendStepOne {
    private_key: String,
}
```

##### `AggSendStepTwoToken`
MPC Step 2: Create partial signature.
```rust
AggSendStepTwoToken {
    #[clap(long)]
    private_key: String,
    #[clap(long)]
    mint: Pubkey,
    #[clap(long)]
    amount: u64,
    #[clap(long)]
    decimals: u8,
    #[clap(long)]
    to: Pubkey,
    #[clap(long)]
    recent_block_hash: String,
    #[clap(long, value_delimiter = ',')]
    keys: Vec<Pubkey>,
    #[clap(long, value_delimiter = ',')]
    first_messages: Vec<String>,
    #[clap(long)]
    secret_state: String,
    #[clap(default_value = "localnet", long)]
    net: Network,
}
```

##### `AggregateSignaturesAndBroadcastToken`
MPC Step 3: Aggregate and broadcast.
```rust
AggregateSignaturesAndBroadcastToken {
    #[clap(long, value_delimiter = ',')]
    signatures: Vec<String>,
    #[clap(long)]
    mint: Pubkey,
    #[clap(long)]
    amount: u64,
    #[clap(long)]
    decimals: u8,
    #[clap(long)]
    to: Pubkey,
    #[clap(long)]
    recent_block_hash: String,
    #[clap(long, value_delimiter = ',')]
    keys: Vec<Pubkey>,
    #[clap(default_value = "localnet", long)]
    net: Network,
}
```

#### `Network`

Enumeration of supported Solana networks.

**Variants:**
- `Mainnet`: Production network
- `Testnet`: Stable testing network  
- `Devnet`: Development network
- `Localnet`: Local validator

**Methods:**

##### `get_cluster_url(&self) -> &'static str`
Returns the RPC endpoint URL for the network.

**Returns:**
- Mainnet: `"https://api.mainnet-beta.solana.com"`
- Testnet: `"https://api.testnet.solana.com"`
- Devnet: `"https://api.devnet.solana.com"`
- Localnet: `"http://127.0.0.1:8899"`

---

## tss.rs

**Purpose**: Implements the MuSig2 threshold signature scheme for multi-party computation.

### Functions

#### `agg_key_to_pubkey(agg_key: &musig2::PublicKeyAgg) -> Pubkey`

Helper function to convert MuSig2 aggregated key to Solana pubkey format.

**Parameters:**
- `agg_key`: MuSig2 aggregated public key

**Returns:**
- `Pubkey`: Solana-compatible public key

**Implementation Details:**
1. Extract compressed point bytes from aggregated key
2. Copy to 32-byte array
3. Create Solana `Pubkey` from bytes

#### `key_agg(keys: Vec<Pubkey>, key: Option<Pubkey>) -> Result<musig2::PublicKeyAgg, Error>`

Aggregates multiple public keys using MuSig2 key aggregation.

**Parameters:**
- `keys`: Vector of public keys to aggregate
- `key`: Optional specific key for coefficient calculation

**Returns:**
- `Ok(PublicKeyAgg)`: Successfully aggregated key
- `Err(Error)`: Key aggregation failure

**Process:**
1. Convert Solana pubkeys to Ed25519 points
2. Compute aggregation coefficients
3. Perform MuSig2 key aggregation
4. Return aggregated key structure

**Security Notes:**
- Uses coefficients to prevent rogue key attacks
- All keys must be valid Ed25519 points

#### `step_one(keypair: Keypair) -> (AggMessage1, SecretAggStepOne)`

Generates nonces and commitments for MPC signing (first round).

**Parameters:**
- `keypair`: Participant's keypair

**Returns:**
- `AggMessage1`: Public message to share with other parties
- `SecretAggStepOne`: Secret state to keep private

**Process:**
1. Create expanded keypair from private key
2. Generate two random nonces using secure RNG
3. Compute public nonce commitments
4. Return public and private components

**Security Notes:**
- Nonces MUST be random and unique per session
- Secret state must be kept private until step 2

#### `step_two_token(...) -> Result<PartialSignature, Error>`

Creates a partial signature for token transfer operations.

**Full Signature:**
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

**Parameters:**
- `keypair`: Participant's keypair
- `mint`: Token mint address
- `amount`: Transfer amount in smallest units
- `decimals`: Token decimal places
- `to`: Recipient address
- `recent_block_hash`: Fresh blockhash
- `keys`: All participant public keys
- `first_messages`: Public messages from step 1
- `secret_state`: Private state from step 1
- `rpc_client`: Solana RPC client

**Returns:**
- `Ok(PartialSignature)`: Partial signature for the transaction
- `Err(Error)`: Signing failure

**Process:**
1. Extract nonces from first messages
2. Aggregate keys and compute coefficients
3. Construct unsigned token transaction
4. Create custom `PartialSigner`
5. Sign transaction and extract partial signature

#### `sign_and_broadcast_token(...) -> Result<Transaction, Error>`

Aggregates partial signatures and creates final transaction.

**Full Signature:**
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

**Parameters:**
- `mint`: Token mint address
- `amount`: Transfer amount (must match step 2)
- `decimals`: Token decimals (must match step 2)
- `to`: Recipient address (must match step 2)
- `recent_block_hash`: Blockhash (must match step 2)
- `keys`: Participant keys (must match step 2)
- `signatures`: Partial signatures from step 2
- `rpc_client`: Solana RPC client

**Returns:**
- `Ok(Transaction)`: Signed transaction ready for broadcast
- `Err(Error)`: Aggregation failure

**Process:**
1. Verify R components match across signatures
2. Deserialize signature components
3. Aggregate partial s values
4. Construct final signature
5. Create signed transaction
6. Verify transaction validity

#### `create_unsigned_token_transaction(...) -> Result<Transaction, Error>`

Creates an unsigned token transfer transaction.

**Full Signature:**
```rust
pub fn create_unsigned_token_transaction(
    mint: Pubkey,
    amount: u64,
    decimals: u8,
    to: &Pubkey,
    payer: &Pubkey,
    rpc_client: &RpcClient,
) -> Result<Transaction, Error>
```

**Parameters:**
- `mint`: Token mint address
- `amount`: Transfer amount in smallest units
- `decimals`: Token decimal places
- `to`: Recipient address
- `payer`: Transaction payer (aggregated key)
- `rpc_client`: Solana RPC client

**Returns:**
- `Ok(Transaction)`: Unsigned transaction
- `Err(Error)`: Transaction construction failure

**Process:**
1. Calculate source and destination ATAs
2. Check if destination ATA exists
3. Add ATA creation instruction if needed
4. Add token transfer instruction
5. Create unsigned transaction

### Structs

#### `PartialSigner`

Custom signer implementation for MuSig2 partial signing.

**Fields:**
```rust
struct PartialSigner {
    signer_private_nonce: musig2::PrivatePartialNonces,
    signer_public_nonce: musig2::PublicPartialNonces,
    other_nonces: Vec<[Point<Ed25519>; 2]>,
    extended_keypair: ExpandedKeyPair,
    aggregated_pubkey: musig2::PublicKeyAgg,
}
```

**Trait Implementations:**

##### `Signer::try_pubkey(&self) -> Result<Pubkey, SignerError>`
Returns the aggregated public key as a Solana `Pubkey`.

##### `Signer::try_sign_message(&self, message: &[u8]) -> Result<Signature, SignerError>`
Creates a partial signature using MuSig2 protocol.

##### `Signer::is_interactive(&self) -> bool`
Returns `false` - this signer doesn't require user interaction.

---

## token.rs

**Purpose**: Handles SPL token operations on Solana blockchain.

### Functions

#### `create_token_mint(...) -> Result<(Pubkey, Signature), Error>`

Creates a new SPL token mint account.

**Full Signature:**
```rust
pub fn create_token_mint(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
) -> Result<(Pubkey, Signature), Error>
```

**Parameters:**
- `rpc_client`: Solana RPC client
- `payer`: Account paying for transaction fees
- `mint_authority`: Authority that can mint tokens
- `freeze_authority`: Optional authority that can freeze accounts
- `decimals`: Number of decimal places (0-9)

**Returns:**
- `Ok((Pubkey, Signature))`: Mint address and transaction signature
- `Err(Error)`: Mint creation failure

**Process:**
1. Generate new keypair for mint account
2. Calculate rent exemption amount
3. Create account creation instruction
4. Create mint initialization instruction
5. Sign and send transaction

#### `mint_tokens_to(...) -> Result<Signature, Error>`

Mints tokens to a destination account.

**Full Signature:**
```rust
pub fn mint_tokens_to(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    destination: &Pubkey,
    mint_authority: &Keypair,
    amount: u64,
    decimals: u8,
) -> Result<Signature, Error>
```

**Parameters:**
- `rpc_client`: Solana RPC client
- `payer`: Account paying transaction fees
- `mint`: Token mint address
- `destination`: Recipient wallet address
- `mint_authority`: Authority keypair that can mint
- `amount`: Amount to mint in smallest units
- `decimals`: Token decimal places

**Returns:**
- `Ok(Signature)`: Transaction signature
- `Err(Error)`: Minting failure

**Process:**
1. Calculate destination ATA address
2. Check if destination ATA exists
3. Create ATA if necessary
4. Create mint instruction
5. Sign and send transaction

#### `transfer_tokens(...) -> Result<Signature, Error>`

Transfers tokens between wallets.

**Full Signature:**
```rust
pub fn transfer_tokens(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    from_wallet: &Keypair,
    to_wallet: &Pubkey,
    amount: u64,
) -> Result<Signature, Error>
```

**Parameters:**
- `rpc_client`: Solana RPC client
- `payer`: Account paying transaction fees
- `mint`: Token mint address
- `from_wallet`: Sender keypair
- `to_wallet`: Recipient wallet address
- `amount`: Amount to transfer in smallest units

**Returns:**
- `Ok(Signature)`: Transaction signature
- `Err(Error)`: Transfer failure

**Process:**
1. Calculate source and destination ATAs
2. Verify source ATA exists
3. Create destination ATA if needed
4. Create transfer instruction
5. Sign and send transaction

#### `get_token_balance(rpc_client: &RpcClient, wallet: &Pubkey, mint: &Pubkey) -> Result<u64, Error>`

Queries token balance for a wallet.

**Parameters:**
- `rpc_client`: Solana RPC client
- `wallet`: Wallet address to check
- `mint`: Token mint address

**Returns:**
- `Ok(u64)`: Token balance in smallest units
- `Err(Error)`: Balance query failure

**Process:**
1. Calculate wallet's ATA address
2. Query token account balance
3. Parse balance amount
4. Return 0 if account doesn't exist

---

## serialization.rs

**Purpose**: Handles serialization/deserialization of MPC protocol messages.

### Traits

#### `Serialize`

Common interface for serializing MPC data structures.

**Methods:**

##### `serialize_bs58(&self) -> String`
Serializes to base58-encoded string for command-line usage.

##### `serialize(&self, append_to: &mut Vec<u8>)`
Serializes to binary format.

##### `deserialize_bs58(s: impl AsRef<[u8]>) -> Result<Self, Error>`
Deserializes from base58-encoded string.

##### `deserialize(b: &[u8]) -> Result<Self, Error>`
Deserializes from binary format.

##### `size_hint(&self) -> usize`
Returns estimated serialized size for memory allocation.

### Enums

#### `Tag`

Message type identifiers for serialized data.

**Variants:**
- `AggMessage1 = 1`: First round MPC message
- `PartialSignature = 2`: Partial signature
- `SecretAggStepOne = 3`: Secret state from step 1

#### `Error`

Serialization-specific error types.

**Variants:**
- `InputTooShort { expected: usize, found: usize }`: Insufficient input data
- `BadBase58(bs58::decode::Error)`: Base58 decoding error
- `InvalidPoint(PointFromBytesError)`: Invalid elliptic curve point
- `InvalidScalar(DeserializationError)`: Invalid scalar value
- `WrongTag { expected: Tag, found: Tag }`: Incorrect message type

### Structs

#### `AggMessage1`

First round message containing public nonces.

**Fields:**
```rust
pub struct AggMessage1 {
    pub public_nonces: PublicPartialNonces,  // [R1, R2]
    pub sender: Pubkey,                      // Sender's public key
}
```

**Serialization Format:**
```
[Tag(1)] [R1(32)] [R2(32)] [Sender(32)] = 97 bytes
```

#### `SecretAggStepOne`

Private state from first round (kept secret).

**Fields:**
```rust
pub struct SecretAggStepOne {
    pub private_nonces: PrivatePartialNonces,  // [r1, r2]
    pub public_nonces: PublicPartialNonces,    // [R1, R2]
}
```

**Serialization Format:**
```
[Tag(3)] [r1(32)] [r2(32)] [R1(32)] [R2(32)] = 129 bytes
```

#### `PartialSignature`

Wrapper around Solana signature for partial signatures.

**Fields:**
```rust
pub struct PartialSignature(pub Signature);  // 64-byte signature
```

**Serialization Format:**
```
[Tag(2)] [Signature(64)] = 65 bytes
```

---

## error.rs

**Purpose**: Centralized error handling for all modules.

### Enum

#### `Error`

Main error type covering all possible failure modes.

**Variants:**

##### Network Errors
- `WrongNetwork(String)`: Invalid network specification
- `BalaceFailed(ClientError)`: Balance query failure
- `AirdropFailed(ClientError)`: Airdrop request failure
- `RecentHashFailed(ClientError)`: Blockhash query failure
- `ConfirmingTransactionFailed(ClientError)`: Transaction confirmation failure

##### Cryptographic Errors
- `BadBase58(Bs58Error)`: Base58 decoding error
- `WrongKeyPair(SignatureError)`: Invalid keypair format
- `PointDeserializationFailed { error, field_name }`: Elliptic curve point error
- `ScalarDeserializationFailed { error, field_name }`: Scalar value error
- `InvalidSignature`: Signature verification failure

##### Protocol Errors
- `KeyPairIsNotInKeys`: Keypair not in aggregation set
- `MismatchMessages`: Inconsistent MPC messages

##### Token Errors
- `TokenCreationFailed(String)`: Token mint creation error
- `TokenMintFailed(String)`: Token minting error
- `TokenTransferFailed(String)`: Token transfer error
- `TokenAccountNotFound`: Required token account missing

##### General Errors
- `FileReadError(String)`: General parsing/input error

### Trait Implementations

#### `Display for Error`
Provides user-friendly error messages.

#### `From<T> for Error`
Automatic conversions from underlying error types:
- `From<Bs58Error>`
- `From<SignatureError>`

#### `std::error::Error for Error`
Standard error trait implementation.

---

## 🔍 Usage Examples

### Key Aggregation
```rust
use solana_mpc_tokens::tss;

let keys = vec![pubkey1, pubkey2, pubkey3];
let agg_key = tss::key_agg(keys, None)?;
let solana_pubkey = tss::agg_key_to_pubkey(&agg_key);
```

### MPC Signing Flow
```rust
// Step 1: Generate nonces
let (public_msg, secret_state) = tss::step_one(keypair);

// Step 2: Create partial signature  
let partial_sig = tss::step_two_token(
    keypair, mint, amount, decimals, to, 
    recent_hash, keys, first_messages, 
    secret_state, &rpc_client
)?;

// Step 3: Aggregate signatures
let tx = tss::sign_and_broadcast_token(
    mint, amount, decimals, to, recent_hash,
    keys, signatures, &rpc_client
)?;
```

### Token Operations
```rust
use solana_mpc_tokens::token;

// Create token mint
let (mint_addr, sig) = token::create_token_mint(
    &rpc_client, &payer, &authority, None, 6
)?;

// Mint tokens
let sig = token::mint_tokens_to(
    &rpc_client, &payer, &mint_addr, &destination,
    &mint_authority, 1000000, 6
)?;

// Check balance
let balance = token::get_token_balance(
    &rpc_client, &wallet, &mint_addr
)?;
```

---

**Next**: Explore [Usage Examples](./usage-examples.md) for practical scenarios or [Development Guide](./development-guide.md) for contribution guidelines. 