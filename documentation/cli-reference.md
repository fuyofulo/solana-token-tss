# CLI Reference Guide

This document provides a complete reference for all command-line interface commands available in Solana MPC Tokens. Each command is documented with syntax, parameters, examples, and use cases.

## 🚀 Basic Usage

```bash
cargo run -- <COMMAND> [OPTIONS]
```

## 📋 Command Categories

- **[Key Management](#-key-management)**: Generate and aggregate keys
- **[SOL Operations](#-sol-operations)**: Balance, airdrop, and blockhash operations  
- **[Token Operations](#-token-operations)**: Create, mint, transfer, and query tokens
- **[MPC Protocol](#-mpc-protocol)**: Multi-party computation commands

---

## 🔑 Key Management

### `generate`

Generate a new cryptographic keypair for use in MPC operations.

**Syntax:**
```bash
cargo run -- generate
```

**Output:**
```
secret share (base58): <private_key_base58>
public key: <public_key>
```

**Example:**
```bash
$ cargo run -- generate
secret share (base58): 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x
public key: 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2
```

**Use Cases:**
- Creating new keypairs for MPC participants
- Setting up test accounts for development
- Generating keys for token authorities

### `aggregate-keys`

Aggregate multiple public keys into a single MPC-controlled public key.

**Syntax:**
```bash
cargo run -- aggregate-keys <KEY1> <KEY2> [KEY3...]
```

**Parameters:**
- `KEY1, KEY2, ...`: Public keys to aggregate (minimum 2 required)

**Example:**
```bash
$ cargo run -- aggregate-keys \
  7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2 \
  9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f

The Aggregated Public Key: AggK3eY5rN8MpL2QvF6XwH9ZjT1UcB4DsG7RmA3VpE8Y
```

**Use Cases:**
- Setting up multi-party wallets
- Creating shared token authorities
- Configuring MPC-controlled accounts

---

## 💰 SOL Operations

### `balance`

Check the SOL balance of any Solana address.

**Syntax:**
```bash
cargo run -- balance <ADDRESS> [--net <NETWORK>]
```

**Parameters:**
- `ADDRESS`: The Solana public key to check
- `--net`: Network to use (default: testnet)
  - Options: `mainnet`, `testnet`, `devnet`, `localnet`

**Example:**
```bash
$ cargo run -- balance 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2 --net devnet
The balance of 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2 is: 5000000000
```

**Use Cases:**
- Checking account balances before operations
- Verifying sufficient SOL for transaction fees
- Monitoring account funding

### `airdrop`

Request SOL from a faucet (testnet/devnet only).

**Syntax:**
```bash
cargo run -- airdrop --to <ADDRESS> --amount <SOL_AMOUNT> [--net <NETWORK>]
```

**Parameters:**
- `--to`: Recipient address for the airdrop
- `--amount`: Amount of SOL to request (decimal)
- `--net`: Network to use (default: testnet)

**Example:**
```bash
$ cargo run -- airdrop \
  --to 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2 \
  --amount 2.5 \
  --net devnet

Airdrop transaction ID: 3x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2
```

**Use Cases:**
- Funding test accounts
- Getting SOL for transaction fees
- Setting up development environments

### `recent-block-hash`

Fetch the most recent blockhash from the network.

**Syntax:**
```bash
cargo run -- recent-block-hash [--net <NETWORK>]
```

**Parameters:**
- `--net`: Network to use (default: testnet)

**Example:**
```bash
$ cargo run -- recent-block-hash --net devnet
Recent blockhash: 8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5
```

**Use Cases:**
- Getting blockhashes for MPC operations
- Checking network connectivity
- Preparing transaction parameters

---

## 🪙 Token Operations

### `create-token`

Create a new SPL token mint with specified parameters.

**Syntax:**
```bash
cargo run -- create-token \
  --mint-authority-key <PRIVATE_KEY_OR_GENERATE> \
  [--freeze-authority-key <PRIVATE_KEY>] \
  [--decimals <DECIMALS>] \
  [--net <NETWORK>]
```

**Parameters:**
- `--mint-authority-key`: Private key for mint authority, or "generate" to create new
- `--freeze-authority-key`: Optional private key for freeze authority
- `--decimals`: Number of decimal places (0-9, default: 6)
- `--net`: Network to use (default: localnet)

**Example:**
```bash
$ cargo run -- create-token \
  --mint-authority-key generate \
  --decimals 8 \
  --net devnet

Generated new mint authority:
Private key (base58): 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x
Public key: 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2

Token mint created successfully!
Mint address: TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2
Transaction signature: 4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1
```

**Use Cases:**
- Creating new token projects
- Setting up test tokens for development
- Establishing MPC-controlled token mints

### `mint-tokens`

Mint tokens to a specified wallet address.

**Syntax:**
```bash
cargo run -- mint-tokens \
  --mint <MINT_ADDRESS> \
  --mint-authority-key <PRIVATE_KEY> \
  --to <RECIPIENT_ADDRESS> \
  --amount <AMOUNT> \
  [--decimals <DECIMALS>] \
  [--net <NETWORK>]
```

**Parameters:**
- `--mint`: Token mint address
- `--mint-authority-key`: Private key of the mint authority
- `--to`: Recipient wallet address
- `--amount`: Amount to mint (in smallest units)
- `--decimals`: Token decimals (default: 6)
- `--net`: Network to use (default: localnet)

**Example:**
```bash
$ cargo run -- mint-tokens \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --mint-authority-key 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x \
  --to 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --amount 1000000 \
  --decimals 6

Tokens minted successfully!
Mint: TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2
To: 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f
Amount: 1000000 tokens
Transaction signature: 6x9f2n4QvR8LmK1ZjT5UcB3DsG8WpA7VeN2YhMrF9qL6
```

**Use Cases:**
- Initial token distribution
- Providing liquidity to test accounts
- Creating token supply for operations

### `transfer-tokens`

Transfer tokens from one wallet to another (single-party operation).

**Syntax:**
```bash
cargo run -- transfer-tokens \
  --mint <MINT_ADDRESS> \
  --from-key <SENDER_PRIVATE_KEY> \
  --to <RECIPIENT_ADDRESS> \
  --amount <AMOUNT> \
  [--net <NETWORK>]
```

**Parameters:**
- `--mint`: Token mint address
- `--from-key`: Private key of the sender
- `--to`: Recipient wallet address
- `--amount`: Amount to transfer (in smallest units)
- `--net`: Network to use (default: localnet)

**Example:**
```bash
$ cargo run -- transfer-tokens \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --from-key 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x \
  --to 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --amount 500000

Token transfer successful!
From: 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2
To: 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f
Amount: 500000 tokens
Transaction signature: 8x1f4n6QvR2LmK7ZjT9UcB1DsG4WpA8VeN5YhMrF3qL9
```

**Use Cases:**
- Simple token transfers
- Moving tokens between personal wallets
- Testing token functionality

### `token-balance`

Check the token balance for a specific wallet and mint.

**Syntax:**
```bash
cargo run -- token-balance \
  --mint <MINT_ADDRESS> \
  --wallet <WALLET_ADDRESS> \
  [--net <NETWORK>]
```

**Parameters:**
- `--mint`: Token mint address
- `--wallet`: Wallet address to check
- `--net`: Network to use (default: localnet)

**Example:**
```bash
$ cargo run -- token-balance \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --wallet 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f

Token balance for wallet 9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f: 1500000 tokens
```

**Use Cases:**
- Verifying token balances
- Checking transfer results
- Monitoring account holdings

---

## 🔐 MPC Protocol

### `agg-send-step-one`

Generate nonces and commitments for MPC signing (Step 1 of 3).

**Syntax:**
```bash
cargo run -- agg-send-step-one <PRIVATE_KEY>
```

**Parameters:**
- `PRIVATE_KEY`: Your private key in base58 format

**Example:**
```bash
$ cargo run -- agg-send-step-one 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x

secret share: SecretStep1_8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5
public share: PublicMsg1_4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1
```

**⚠️ Important:**
- Keep the `secret share` private - never share it
- Share the `public share` with all other parties
- Each party must complete this step

**Use Cases:**
- Initiating MPC token transfers
- Starting multi-party signing sessions
- Setting up threshold signature operations

### `agg-send-step-two-token`

Create partial signature for token transfer (Step 2 of 3).

**Syntax:**
```bash
cargo run -- agg-send-step-two-token \
  --private-key <PRIVATE_KEY> \
  --mint <MINT_ADDRESS> \
  --amount <AMOUNT> \
  --decimals <DECIMALS> \
  --to <RECIPIENT_ADDRESS> \
  --recent-block-hash <BLOCKHASH> \
  --keys <KEY1,KEY2,...> \
  --first-messages <MSG1,MSG2,...> \
  --secret-state <SECRET_FROM_STEP1> \
  [--net <NETWORK>]
```

**Parameters:**
- `--private-key`: Your private key in base58 format
- `--mint`: Token mint address
- `--amount`: Amount to transfer (in smallest units)
- `--decimals`: Token decimals
- `--to`: Recipient wallet address
- `--recent-block-hash`: Recent blockhash from network
- `--keys`: Comma-separated list of all participant public keys
- `--first-messages`: Comma-separated list of public shares from step 1
- `--secret-state`: Your secret share from step 1
- `--net`: Network to use (default: localnet)

**Example:**
```bash
$ cargo run -- agg-send-step-two-token \
  --private-key 5J3mBbAH58CpQ3Y2BbhbRX8k9ZrGZ4WjgXGhL9Cp2PQnY8vN4kR3x \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --amount 1000000 \
  --decimals 6 \
  --to RecipientAddr8x1f4n6QvR2LmK7ZjT9UcB1DsG4WpA8VeN5YhMrF3qL9 \
  --recent-block-hash 8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5 \
  --keys 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2,9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f \
  --first-messages PublicMsg1_4x7f8n1QvR6LmK8ZjT3UcB5DsG7WpA4VeN9YhMrF6qL1,PublicMsg2_6x9f2n4QvR8LmK1ZjT5UcB3DsG8WpA7VeN2YhMrF9qL6 \
  --secret-state SecretStep1_8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5

partial signature: PartialSig_2x5f9n7QvR4LmK3ZjT8UcB6DsG1WpA9VeN6YhMrF2qL8
```

**⚠️ Important:**
- All parties must use identical transaction parameters
- The recent blockhash must be fresh (not expired)
- All first messages must be included in correct order

**Use Cases:**
- Creating partial signatures for token transfers
- Participating in multi-party transactions
- Contributing to threshold signatures

### `aggregate-signatures-and-broadcast-token`

Combine partial signatures and broadcast transaction (Step 3 of 3).

**Syntax:**
```bash
cargo run -- aggregate-signatures-and-broadcast-token \
  --signatures <SIG1,SIG2,...> \
  --mint <MINT_ADDRESS> \
  --amount <AMOUNT> \
  --decimals <DECIMALS> \
  --to <RECIPIENT_ADDRESS> \
  --recent-block-hash <BLOCKHASH> \
  --keys <KEY1,KEY2,...> \
  [--net <NETWORK>]
```

**Parameters:**
- `--signatures`: Comma-separated list of partial signatures from step 2
- `--mint`: Token mint address (must match step 2)
- `--amount`: Amount to transfer (must match step 2)
- `--decimals`: Token decimals (must match step 2)
- `--to`: Recipient address (must match step 2)
- `--recent-block-hash`: Recent blockhash (must match step 2)
- `--keys`: Participant public keys (must match step 2)
- `--net`: Network to use (default: localnet)

**Example:**
```bash
$ cargo run -- aggregate-signatures-and-broadcast-token \
  --signatures PartialSig_2x5f9n7QvR4LmK3ZjT8UcB6DsG1WpA9VeN6YhMrF2qL8,PartialSig_7x2f1n9QvR5LmK6ZjT1UcB9DsG3WpA2VeN8YhMrF5qL4 \
  --mint TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2 \
  --amount 1000000 \
  --decimals 6 \
  --to RecipientAddr8x1f4n6QvR2LmK7ZjT9UcB1DsG4WpA8VeN5YhMrF3qL9 \
  --recent-block-hash 8x9f6n3QvR8LmK2ZjT4UcB7DsG9WpA6VeN1YhMrF8qL5 \
  --keys 7YhBqApRkxgD4jF3WnBw2Tk5VxGqHhkLn6zrDvQm8Ux2,9ZpCrFg5nKj8Hs2MvL6TpR4QuW3XeN7YmA9BxD1VsE8f

Token transfer successful!
Transaction ID: FinalTx_9x3f7n2QvR1LmK4ZjT6UcB8DsG5WpA1VeN3YhMrF7qL6
Mint: TokenMint9x8f5n2QvR7LmK9ZjT1UcB4DsG6WpA3VeN8YhMrF5qL2
To: RecipientAddr8x1f4n6QvR2LmK7ZjT9UcB1DsG4WpA8VeN5YhMrF3qL9
Amount: 1000000 tokens
```

**⚠️ Important:**
- All parameters must exactly match those used in step 2
- All partial signatures must be collected before running this command
- Anyone can execute this final step

**Use Cases:**
- Finalizing multi-party token transfers
- Broadcasting threshold signature transactions
- Completing MPC operations

---

## 🎯 Common Workflows

### Complete MPC Token Transfer (2-Party)

```bash
# 1. Both parties: Generate nonces
cargo run -- agg-send-step-one $PRIVATE_KEY_1  # Party 1
cargo run -- agg-send-step-one $PRIVATE_KEY_2  # Party 2

# 2. Get recent blockhash
cargo run -- recent-block-hash --net devnet

# 3. Both parties: Create partial signatures (using outputs from steps 1-2)
cargo run -- agg-send-step-two-token --private-key $PRIVATE_KEY_1 ...  # Party 1
cargo run -- agg-send-step-two-token --private-key $PRIVATE_KEY_2 ...  # Party 2

# 4. Anyone: Aggregate and broadcast (using outputs from step 3)
cargo run -- aggregate-signatures-and-broadcast-token --signatures $SIG1,$SIG2 ...
```

### Setting Up New Token with MPC Control

```bash
# 1. Generate keypairs for all parties
cargo run -- generate  # Party 1
cargo run -- generate  # Party 2

# 2. Create aggregated authority
cargo run -- aggregate-keys $PUBKEY_1 $PUBKEY_2

# 3. Create token with aggregated authority
cargo run -- create-token --mint-authority-key $AGG_PRIVATE_KEY --net devnet

# 4. Now use MPC protocol for all token operations
```

---

**Next**: See [Usage Examples](./usage-examples.md) for practical scenarios or [API Reference](./api-reference.md) for technical details. 