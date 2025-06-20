# Development Guide

This guide is for developers who want to contribute to the Solana MPC Tokens project or understand the codebase at a deeper level.

## 🎯 Developer Setup

### Prerequisites

- **Rust**: 1.70.0 or newer with `rustfmt` and `clippy`
- **Git**: For version control
- **VSCode** (recommended) with Rust extensions
- **Solana CLI**: For local testing (optional)

### Development Environment

```bash
# Clone the repository
git clone https://github.com/your-org/solana-mpc-tokens.git
cd solana-mpc-tokens

# Install Rust components
rustup component add rustfmt clippy

# Build in development mode
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linting
cargo clippy -- -D warnings
```

## 📁 Project Structure

```
solana-mpc-tokens/
├── src/
│   ├── main.rs              # Application entry point
│   ├── cli.rs               # Command-line interface
│   ├── tss.rs              # MuSig2 implementation
│   ├── token.rs            # SPL token operations
│   ├── serialization.rs    # MPC message serialization
│   └── error.rs            # Error handling
├── documentation/          # Documentation files
├── target/                # Build artifacts
├── Cargo.toml             # Dependencies and metadata
└── Cargo.lock             # Dependency lock file
```

## 🔧 Core Dependencies

### Cryptography
- **`multi_party_eddsa`**: MuSig2 protocol implementation
- **`curv`**: Elliptic curve cryptography primitives
- **`ed25519-dalek`**: Ed25519 signature scheme

### Solana Integration
- **`solana-sdk`**: Core Solana types and utilities
- **`solana-client`**: RPC client for blockchain interaction
- **`spl-token`**: SPL token program bindings
- **`spl-associated-token-account`**: Associated token account utilities

### CLI and Serialization
- **`clap`**: Command-line argument parsing
- **`bs58`**: Base58 encoding/decoding
- **`serde`**: Serialization framework

## 🏗️ Architecture Patterns

### Error Handling Strategy

The codebase follows a consistent error handling pattern:

```rust
// Custom error type that wraps all possible errors
#[derive(Debug)]
pub enum Error {
    // Network-related errors
    WrongNetwork(String),
    BalaceFailed(ClientError),
    
    // Cryptographic errors
    BadBase58(Bs58Error),
    WrongKeyPair(SignatureError),
    
    // Protocol errors
    InvalidSignature,
    MismatchMessages,
}

// Automatic conversion from underlying errors
impl From<Bs58Error> for Error {
    fn from(err: Bs58Error) -> Self {
        Error::BadBase58(err)
    }
}

// Use Result<T, Error> consistently
pub fn my_function() -> Result<String, Error> {
    let decoded = bs58::decode(input)?;  // Automatic conversion
    Ok(format!("Success: {}", decoded.len()))
}
```

### Module Organization

Each module has a clear responsibility:

- **`main.rs`**: Minimal entry point, delegates to modules
- **`cli.rs`**: Pure command definition, no business logic
- **`tss.rs`**: MPC protocol implementation, crypto-heavy
- **`token.rs`**: Solana/SPL integration, blockchain interaction
- **`serialization.rs`**: Message format handling, I/O concerns
- **`error.rs`**: Centralized error definitions

### Type Safety

The codebase leverages Rust's type system for safety:

```rust
// Wrapper types prevent mixing up similar values
pub struct PartialSignature(pub Signature);
pub struct AggMessage1 { /* ... */ }

// Enums with data for tagged unions
pub enum Tag {
    AggMessage1 = 1,
    PartialSignature = 2,
    SecretAggStepOne = 3,
}

// Option types for optional parameters
pub fn create_token_mint(
    // ...
    freeze_authority: Option<&Pubkey>,  // Clear optionality
    // ...
) -> Result<(Pubkey, Signature), Error>
```

## 🧪 Testing Strategy

### Unit Tests

Each module includes unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_aggregation() {
        let keys = vec![
            /* test pubkeys */
        ];
        let result = key_agg(keys, None);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_serialization_roundtrip() {
        let original = AggMessage1 { /* ... */ };
        let serialized = original.serialize_bs58();
        let deserialized = AggMessage1::deserialize_bs58(&serialized).unwrap();
        // Assert equality
    }
}
```

### Integration Tests

Test complete workflows:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_key_aggregation

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Manual Testing

Use the provided test scripts:

```bash
# Test MPC protocol end-to-end
./mpc_token_test.sh

# Test devnet operations
./devnet_mpc_test.sh

# Manual testing on specific network
./manual_mpc_test.sh
```

## 🔐 Security Considerations

### Cryptographic Security

1. **Nonce Generation**: Always use secure random number generation
2. **Key Handling**: Never log or print private keys
3. **Memory Safety**: Rust prevents many memory-related vulnerabilities
4. **Input Validation**: Validate all external inputs

### Code Review Checklist

- [ ] **No Private Key Exposure**: Keys never logged or serialized inappropriately
- [ ] **Proper Error Handling**: All Results are handled, no `unwrap()` in production code
- [ ] **Input Validation**: External inputs are validated before use
- [ ] **Constant Time Operations**: Cryptographic operations use constant-time implementations
- [ ] **Fresh Randomness**: Nonces and random values are generated securely

### Security Testing

```rust
#[test]
fn test_no_nonce_reuse() {
    let keypair = generate_keypair();
    let (msg1_a, secret_a) = step_one(keypair.clone());
    let (msg1_b, secret_b) = step_one(keypair.clone());
    
    // Ensure nonces are different
    assert_ne!(msg1_a.public_nonces, msg1_b.public_nonces);
}

#[test]
fn test_signature_verification() {
    // Create partial signatures
    let partials = create_partial_signatures();
    
    // Aggregate
    let final_sig = aggregate_signatures(partials)?;
    
    // Verify the final signature is valid
    assert!(verify_signature(&final_sig, &message, &pubkey));
}
```

## 🚀 Building and Release

### Development Builds

```bash
# Fast development build
cargo build

# With debugging symbols
cargo build --profile dev

# Check without building
cargo check
```

### Release Builds

```bash
# Optimized release build
cargo build --release

# Strip symbols for smaller binary
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/solana-mpc-tokens
```

### Cross-Platform Builds

```bash
# Install targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin

# Build for different platforms
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
```

## 📝 Contributing Guidelines

### Code Style

Follow the official Rust style guide:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Lint code
cargo clippy -- -D warnings
```

### Commit Messages

Use conventional commit format:

```
feat: add support for multi-party signatures
fix: resolve nonce reuse vulnerability
docs: update API documentation for tss module
test: add integration tests for token operations
refactor: simplify error handling in cli module
```

### Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/new-feature`
3. **Make** your changes with tests
4. **Ensure** all tests pass: `cargo test`
5. **Format** code: `cargo fmt`
6. **Lint** code: `cargo clippy`
7. **Commit** with descriptive messages
8. **Push** to your fork: `git push origin feature/new-feature`
9. **Create** a pull request

### Documentation

Keep documentation up to date:

```rust
/// Aggregates multiple public keys using MuSig2 protocol.
///
/// # Arguments
/// * `keys` - Vector of public keys to aggregate
/// * `key` - Optional specific key for coefficient calculation
///
/// # Returns
/// * `Ok(PublicKeyAgg)` - Successfully aggregated key
/// * `Err(Error)` - Key aggregation failure
///
/// # Examples
/// ```
/// let keys = vec![pubkey1, pubkey2];
/// let agg_key = key_agg(keys, None)?;
/// ```
pub fn key_agg(keys: Vec<Pubkey>, key: Option<Pubkey>) -> Result<musig2::PublicKeyAgg, Error> {
    // Implementation...
}
```

## 🔍 Debugging Tips

### Logging

Add logging for debugging:

```rust
use log::{debug, info, warn, error};

pub fn step_two_token(/* ... */) -> Result<PartialSignature, Error> {
    debug!("Starting step two with {} keys", keys.len());
    
    let aggkey = key_agg(keys.clone(), Some(keypair.pubkey()))?;
    info!("Key aggregation successful");
    
    // ... rest of implementation
}
```

Enable logging:

```bash
# Set log level
RUST_LOG=debug cargo run -- agg-send-step-two-token ...

# Log to file
RUST_LOG=info cargo run -- ... 2> debug.log
```

### Common Debug Scenarios

#### Signature Verification Failures

```rust
#[derive(Debug)]
pub struct DebugSignature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub message_hash: [u8; 32],
    pub pubkey: [u8; 32],
}

// Add debug output for signature components
let debug_sig = DebugSignature {
    r: signature.r,
    s: signature.s,
    message_hash: hash_message(&transaction),
    pubkey: aggregated_pubkey.to_bytes(),
};
println!("Debug signature: {:#?}", debug_sig);
```

#### Network Issues

```rust
// Add timeout and retry logic
let client = RpcClient::new_with_timeout_and_commitment(
    url,
    Duration::from_secs(30),
    CommitmentConfig::confirmed(),
);

// Test connection
match client.get_latest_blockhash() {
    Ok(hash) => info!("Connected to network, latest hash: {}", hash),
    Err(e) => error!("Network connection failed: {}", e),
}
```

## 🎯 Advanced Development

### Custom Features

Add feature flags in `Cargo.toml`:

```toml
[features]
default = ["solana-integration"]
solana-integration = ["solana-sdk", "solana-client"]
test-utils = []
debug-crypto = []
```

Use in code:

```rust
#[cfg(feature = "debug-crypto")]
fn debug_print_nonces(nonces: &PublicPartialNonces) {
    println!("Debug nonces: {:?}", nonces);
}

#[cfg(not(feature = "solana-integration"))]
compile_error!("This build requires solana-integration feature");
```

### Performance Optimization

Profile the application:

```bash
# Install profiling tools
cargo install cargo-profdata

# Profile with perf
cargo build --release
perf record --call-graph=dwarf ./target/release/solana-mpc-tokens generate
perf report

# Memory profiling with valgrind
cargo build
valgrind --tool=massif ./target/debug/solana-mpc-tokens generate
```

### Extending the Protocol

To add new MPC operations:

1. **Define the message format** in `serialization.rs`
2. **Add CLI commands** in `cli.rs`
3. **Implement the protocol** in `tss.rs`
4. **Add error handling** in `error.rs`
5. **Write comprehensive tests**
6. **Update documentation**

Example for adding SOL transfers:

```rust
// serialization.rs
pub struct SolTransferMessage {
    pub amount: u64,
    pub recipient: Pubkey,
    pub sender_nonces: PublicPartialNonces,
}

// cli.rs
AggSendStepTwoSol {
    #[clap(long)]
    amount: u64,
    #[clap(long)]
    to: Pubkey,
    // ... other fields
}

// tss.rs
pub fn step_two_sol(/* ... */) -> Result<PartialSignature, Error> {
    // Implementation for SOL transfers
}
```

## 🏆 Best Practices

### Code Organization

- **Single Responsibility**: Each function has one clear purpose
- **Dependency Injection**: Pass dependencies as parameters
- **Error Propagation**: Use `?` operator consistently
- **Type Safety**: Use strong types over primitive types
- **Documentation**: Document public APIs thoroughly

### Testing Strategy

- **Unit Tests**: Test individual functions in isolation
- **Integration Tests**: Test complete workflows
- **Property Tests**: Use `proptest` for complex invariants
- **Fuzz Testing**: Test with random inputs

### Security Mindset

- **Validate Everything**: Never trust external input
- **Fail Securely**: Errors should not leak sensitive information
- **Audit Dependencies**: Regularly update and audit dependencies
- **Code Review**: All changes require review
- **Defense in Depth**: Multiple layers of security

---

**Ready to contribute?** Check the [GitHub Issues](https://github.com/your-org/solana-mpc-tokens/issues) for tasks to work on! 