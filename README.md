# Solana MPC Tokens

A robust implementation of Multi-Party Computation (MPC) for managing SPL tokens and SOL on the Solana blockchain. This protocol enables secure, distributed signing of transactions without exposing private keys, making it ideal for high-security token management scenarios.

## Core Features

- **Multi-Party Computation (MPC)**: Implements threshold EdDSA signatures using distributed key generation
- **SPL Token Support**: Full support for creating, minting, and transferring SPL tokens
- **Native SOL Support**: MPC-based transfers of native SOL with memo support
- **Network Flexibility**: Compatible with Solana mainnet, devnet, and localnet
- **CLI Interface**: Comprehensive command-line interface for all MPC operations

## Prerequisites

- Rust toolchain (1.70.0 or later)
- Solana CLI tools (1.16.0 or later)
- For devnet testing: USDC SPL token account
- For localnet testing: Running Solana validator (`solana-test-validator`)

## Installation

```bash
git clone https://github.com/yourusername/solana-mpc-tokens
cd solana-mpc-tokens
cargo build --release
```

## Quick Start

Two demonstration scripts are provided to help you understand the MPC workflow:

### 1. Localnet Full Demo

The `mpc_demo_localnet.sh` script provides a comprehensive demonstration of all MPC capabilities:

```bash
# Start a local validator in a separate terminal
solana-test-validator

# Run the demo
./mpc_demo_localnet.sh
```

This demo walks through:
- MPC participant key generation
- Aggregated wallet creation
- SPL token creation and minting
- MPC-based token transfers
- MPC-based SOL transfers

### 2. Devnet USDC Demo

The `devnet_mpc_test.sh` script demonstrates MPC operations with USDC on devnet:

```bash
./devnet_mpc_test.sh
```

This demo focuses on:
- Using predefined MPC participant keys
- Transferring USDC tokens using MPC signatures
- Working with existing SPL tokens on devnet

## Technical Protocol Overview

### MPC Signing Process

1. **Nonce Generation (Step One)**
   ```bash
   solana-mpc-tokens agg-send-step-one <private-key>
   ```

2. **Partial Signature Creation (Step Two)**
   ```bash
   solana-mpc-tokens agg-send-step-two-token \
     --private-key <key> \
     --mint <address> \
     --amount <amount> \
     --decimals <decimals> \
     --to <recipient> \
     --recent-block-hash <hash> \
     --keys <pubkey1,pubkey2,pubkey3> \
     --first-messages <msg1,msg2> \
     --secret-state <state>
   ```

3. **Signature Aggregation and Broadcasting**
   ```bash
   solana-mpc-tokens aggregate-signatures-and-broadcast-token \
     --signatures <sig1,sig2,sig3> \
     --mint <address> \
     --amount <amount> \
     --decimals <decimals> \
     --to <recipient> \
     --recent-block-hash <hash> \
     --keys <pubkey1,pubkey2,pubkey3>
   ```

### Security Considerations

- Each participant's private key never leaves their secure environment
- Nonces are generated fresh for each signing session
- Partial signatures are useless without the complete set
- Transaction requires all participants' signatures to be valid

## Advanced Usage

### Token Management

```bash
# Create new SPL token
solana-mpc-tokens create-token --mint-authority-key <key> --decimals 6

# Mint tokens
solana-mpc-tokens mint-tokens \
  --mint <address> \
  --mint-authority-key <key> \
  --to <recipient> \
  --amount <amount> \
  --decimals <decimals>

# Check token balance
solana-mpc-tokens token-balance --mint <address> --wallet <address>
```

### SOL Operations

```bash
# MPC-based SOL transfer
solana-mpc-tokens aggregate-signatures-and-broadcast-sol \
  --signatures <sig1,sig2,sig3> \
  --amount <amount> \
  --to <recipient> \
  --memo "Transfer memo" \
  --recent-block-hash <hash> \
  --keys <pubkey1,pubkey2,pubkey3>
```

## Documentation

For more detailed information, refer to:
- **[CLI Reference](./cli-reference.md)** - Complete command-line interface documentation

## Contributing

Contributions are welcome! Please ensure your pull requests adhere to the following:

- Follow Rust coding standards
- Include comprehensive tests
- Update documentation as needed
- Sign your commits

## License

[Insert License Information] 