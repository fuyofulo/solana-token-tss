# Getting Started

Welcome to Solana MPC Tokens! This guide will help you set up the project and run your first multi-party computation operations.

## 🎯 What You'll Learn

By the end of this guide, you'll be able to:
- Set up the development environment
- Generate cryptographic keypairs
- Create and manage SPL tokens
- Perform multi-party token transfers
- Understand the MPC workflow

## 📋 Prerequisites

Before starting, ensure you have:

### System Requirements
- **Operating System**: Linux, macOS, or Windows (with WSL)
- **Memory**: At least 4GB RAM
- **Storage**: 2GB free space

### Software Dependencies
- **Rust**: Version 1.70.0 or newer
- **Git**: For cloning the repository
- **Solana CLI**: For local development (optional but recommended)

## 🔧 Installation

### Step 1: Install Rust

If you don't have Rust installed:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 2: Clone the Repository

```bash
git clone https://github.com/your-org/solana-mpc-tokens.git
cd solana-mpc-tokens
```

### Step 3: Build the Project

```bash
# Build in debug mode (faster compilation)
cargo build

# Or build in release mode (optimized)
cargo build --release
```

### Step 4: Verify Installation

```bash
# Test that the application runs
cargo run -- --help
```

You should see the help output with all available commands.

## 🚀 Quick Start Tutorial

Let's walk through a complete example of creating and transferring tokens using MPC.

### 🔑 Step 1: Generate Keypairs

First, create keypairs for all participants in the MPC protocol.

```bash
# Generate keypair for Party 1
cargo run -- generate
# Output:
# secret share (base58): 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x
# public key: 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2

# Generate keypair for Party 2  
cargo run -- generate
# Output:
# secret share (base58): 4K2mAaH47CpP2Y1BahbRW7k8ZqGY3WjfXGhK8Cp1POnX7vM3jR2w
# public key: 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f
```

⚠️ **Important**: Save these keypairs securely! In a real-world scenario:
- Each party generates their own keypair privately
- Only public keys are shared between parties
- Private keys never leave their owner's possession

### 🔗 Step 2: Create Aggregated Key

Combine the public keys into a single MPC-controlled key:

```bash
cargo run -- aggregate-keys \
  7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2 \
  9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f

# Output:
# The Aggregated Public Key: AggK3eY5rN8MpL2QvF6XwH9ZjT1UcB4DsG7RmA3VpE8Y
```

This aggregated key can now be used as a token authority that requires both parties to cooperate for any operations.

### 🪙 Step 3: Create a Token

Create a new SPL token mint controlled by the aggregated key:

```bash
# First, get some SOL for transaction fees (devnet/testnet only)
cargo run -- airdrop \
  --to AggK3eY5rN8MpL2QvF6XwH9ZjT1UcB4DsG7RmA3VpE8Y \
  --amount 2.0 \
  --net devnet

# Create the token mint (we'll use a regular keypair for simplicity)
cargo run -- create-token \
  --mint-authority-key generate \
  --decimals 6 \
  --net devnet

# Output:
# Generated new mint authority:
# Private key (base58): 3H1mCcG36DpO1Z0CahcQV6j7YpFX2FhJ7Cp0QNnW6wL2jR1v
# Public key: TokenAuth8x9f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2
# 
# Token mint created successfully!
# Mint address: TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2
# Transaction signature: 4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1
```

### 🎯 Step 4: Mint Initial Tokens

Let's mint some tokens to our aggregated address so we can transfer them:

```bash
cargo run -- mint-tokens \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --mint-authority-key 3H1mCcG36DpO1Z0CahcQV6j7YpFX2FhJ7Cp0QNnW6wL2jR1v \
  --to AggK3eY5rN8MpL2QvF6XwH9ZjT1UcB4DsG7RmA3VpE8Y \
  --amount 1000000000 \
  --decimals 6 \
  --net devnet

# Output:
# Tokens minted successfully!
# Amount: 1000000000 tokens (1,000 tokens with 6 decimals)
```

### 🔄 Step 5: MPC Token Transfer

Now comes the exciting part - performing a multi-party token transfer! This requires coordination between both parties.

#### Phase 1: Generate Nonces (Both Parties)

Each party generates cryptographic nonces:

```bash
# Party 1 generates nonces
cargo run -- agg-send-step-one 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x

# Output:
# secret share: SecretStep1_8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5
# public share: PublicMsg1_4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1

# Party 2 generates nonces
cargo run -- agg-send-step-one 4K2mAaH47CpP2Y1BahbRW7k8ZqGY3WjfXGhK8Cp1POnX7vM3jR2w

# Output:  
# secret share: SecretStep2_6x9f2n4QvR8LmK1ZjT5UcB3DsG8WpA7VeN2YhMrF9qL6
# public share: PublicMsg2_9x3f7n2QvR1LmK4ZjT6UcB8DsG5WpA1VeN3YhMrF7qL6
```

#### Phase 2: Get Recent Blockhash

Get a fresh blockhash for the transaction:

```bash
cargo run -- recent-block-hash --net devnet

# Output:
# Recent blockhash: BlockHash_7x2f1n9QvR5LmK6ZjT1UcB9DsG3WpA2VeN8YhMrF5qL4
```

#### Phase 3: Create Partial Signatures (Both Parties)

Each party creates a partial signature for the same transaction:

```bash
# Party 1 creates partial signature
cargo run -- agg-send-step-two-token \
  --private-key 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --amount 500000000 \
  --decimals 6 \
  --to 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --recent-block-hash BlockHash_7x2f1n9QvR5LmK6ZjT1UcB9DsG3WpA2VeN8YhMrF5qL4 \
  --keys 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2,9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --first-messages PublicMsg1_4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1,PublicMsg2_9x3f7n2QvR1LmK4ZjT6UcB8DsG5WpA1VeN3YhMrF7qL6 \
  --secret-state SecretStep1_8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5 \
  --net devnet

# Output:
# partial signature: PartialSig1_2x5f9n7QvR4LmK3ZjT8UcB6DsG1WpA9VeN6YhMrF2qL8

# Party 2 creates partial signature (same parameters!)
cargo run -- agg-send-step-two-token \
  --private-key 4K2mAaH47CpP2Y1BahbRW7k8ZqGY3WjfXGhK8Cp1POnX7vM3jR2w \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --amount 500000000 \
  --decimals 6 \
  --to 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --recent-block-hash BlockHash_7x2f1n9QvR5LmK6ZjT1UcB9DsG3WpA2VeN8YhMrF5qL4 \
  --keys 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2,9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --first-messages PublicMsg1_4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1,PublicMsg2_9x3f7n2QvR1LmK4ZjT6UcB8DsG5WpA1VeN3YhMrF7qL6 \
  --secret-state SecretStep2_6x9f2n4QvR8LmK1ZjT5UcB3DsG8WpA7VeN2YhMrF9qL6 \
  --net devnet

# Output:
# partial signature: PartialSig2_5x8f2n1QvR7LmK9ZjT4UcB1DsG6WpA4VeN9YhMrF8qL3
```

#### Phase 4: Aggregate and Broadcast

Finally, combine the partial signatures and broadcast the transaction:

```bash
cargo run -- aggregate-signatures-and-broadcast-token \
  --signatures PartialSig1_2x5f9n7QvR4LmK3ZjT8UcB6DsG1WpA9VeN6YhMrF2qL8,PartialSig2_5x8f2n1QvR7LmK9ZjT4UcB1DsG6WpA4VeN9YhMrF8qL3 \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --amount 500000000 \
  --decimals 6 \
  --to 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --recent-block-hash BlockHash_7x2f1n9QvR5LmK6ZjT1UcB9DsG3WpA2VeN8YhMrF5qL4 \
  --keys 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2,9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --net devnet

# Output:
# Token transfer successful!
# Transaction ID: FinalTx_1x4f6n8QvR9LmK2ZjT7UcB5DsG2WpA6VeN4YhMrF1qL9
# From: AggK3eY5rN8MpL2QvF6XwH9ZjT1UcB4DsG7RmA3VpE8Y (Aggregated Key)
# To: 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f
# Amount: 500000000 tokens (500 tokens with 6 decimals)
```

🎉 **Congratulations!** You've successfully completed your first multi-party token transfer!

## 🔍 What Just Happened?

Let's break down the magic:

1. **Key Aggregation**: Multiple public keys were combined into a single aggregated key using MuSig2
2. **Nonce Generation**: Each party generated random nonces without revealing them
3. **Partial Signing**: Each party created a partial signature for the exact same transaction
4. **Signature Aggregation**: The partial signatures were combined into a single, valid signature
5. **Transaction Broadcast**: The final transaction appears on-chain as a normal single-signature transaction

### Key Security Properties

- ✅ **No Single Point of Failure**: Both parties must participate
- ✅ **Privacy Preserving**: Individual keys never revealed
- ✅ **On-chain Efficiency**: Looks like a regular transaction
- ✅ **Non-repudiation**: Both parties committed to the transaction

## 📚 Next Steps

Now that you've mastered the basics, explore these advanced topics:

### 🔧 Learn More Commands
- Check out the [CLI Reference](./cli-reference.md) for all available commands
- Explore different network configurations (mainnet, testnet, localnet)

### 🎯 Practical Applications
- Read [Usage Examples](./usage-examples.md) for real-world scenarios
- Learn about [Token Operations](./token-operations.md) in detail

### 🏗️ Understand the Architecture
- Study the [Architecture Overview](./architecture.md)
- Deep dive into the [MPC Protocol](./mpc-protocol.md)

### 🛠️ Development
- Contribute to the project with the [Development Guide](./development-guide.md)
- Run tests with the [Testing Guide](./testing-guide.md)

## ⚠️ Important Security Notes

Before using in production:

1. **Use Secure Communication**: Share MPC messages through authenticated channels
2. **Fresh Nonces**: Never reuse nonces between signing sessions
3. **Verify Transactions**: Always verify transaction details before signing
4. **Key Management**: Implement proper key storage and backup procedures
5. **Network Security**: Use appropriate network endpoints for your use case

## 🆘 Troubleshooting

### Common Issues

#### Build Failures
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build
```

#### Network Connectivity
```bash
# Test network connection
cargo run -- recent-block-hash --net devnet

# Use different RPC endpoint if needed
```

#### Invalid Signatures
- Ensure all parties use identical transaction parameters
- Verify blockhash is fresh (not expired)
- Check that nonces are correctly shared

### Getting Help

- **Documentation**: Check other docs in this folder
- **Issues**: Report bugs on the GitHub repository
- **Community**: Join our developer community forums

---

**Ready to dive deeper?** Continue with [Architecture Overview](./architecture.md) to understand how everything works under the hood! 