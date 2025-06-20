# Architecture Overview

This document provides a comprehensive overview of the Solana MPC Tokens architecture, explaining how the different modules work together to enable multi-party computation for SPL token operations.

## 🏗️ High-Level Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                        Solana MPC Tokens                         │
├──────────────────────────────────────────────────────────────────┤
│  CLI Layer (cli.rs)                                              │
│  ┌─────────────┬─────────────┬─────────────┬─────────────────┐   │
│  │   Generate  │   Balance   │  Token Ops  │   MPC Protocol  │   │
│  │   Commands  │   Commands  │   Commands  │    Commands     │   │
│  └─────────────┴─────────────┴─────────────┴─────────────────┘   │
├──────────────────────────────────────────────────────────────────┤
│  Application Logic (main.rs)                                     │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │              Command Routing & Coordination                 │ │
│  └─────────────────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────────────────┤
│  Core Modules                                                    │
│  ┌─────────────┬─────────────┬─────────────┬─────────────────┐   │
│  │   TSS       │   Token     │ Serialization│     Error      │   │
│  │  (tss.rs)   │ (token.rs)  │(serialization.rs)│ (error.rs) │   │
│  └─────────────┴─────────────┴─────────────┴─────────────────┘   │
├──────────────────────────────────────────────────────────────────┤
│  External Dependencies                                           │
│  ┌─────────────┬─────────────┬─────────────┬─────────────────┐   │
│  │ MuSig2 Lib  │ Solana SDK  │ SPL Token   │   Curv Crypto   │   │
│  └─────────────┴─────────────┴─────────────┴─────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │   Solana Blockchain │
                    │    (RPC Endpoint)   │
                    └─────────────────────┘
```

## 🧩 Module Breakdown

### 1. **CLI Layer (`cli.rs`)**

**Purpose**: Defines the command-line interface and argument parsing.

**Key Components**:
- `Options` enum: Defines all available commands
- `Network` enum: Specifies blockchain networks (mainnet, testnet, devnet, localnet)
- Command validation and argument parsing

**Responsibilities**:
- Parse command-line arguments
- Validate input parameters
- Route commands to appropriate handlers

### 2. **Application Logic (`main.rs`)**

**Purpose**: Central coordinator that handles command execution and orchestrates different modules.

**Key Components**:
- `parse_keypair()`: Converts base58 private keys to Keypair objects
- Command matching and execution logic
- Error handling and user feedback

**Responsibilities**:
- Route parsed commands to appropriate modules
- Handle common operations (key parsing, network connections)
- Coordinate multi-step MPC operations
- Provide user-friendly output

### 3. **Threshold Signature Scheme (`tss.rs`)**

**Purpose**: Implements the core MPC functionality using MuSig2 protocol.

**Key Components**:
- `key_agg()`: Aggregates multiple public keys into a single key
- `step_one()`: Generates nonces for MPC protocol
- `step_two_token()`: Creates partial signatures for token operations
- `sign_and_broadcast_token()`: Aggregates partial signatures and creates final transaction
- `PartialSigner`: Custom signer implementation for Solana transactions

**Flow**:
```
1. Key Aggregation:   [PubKey1, PubKey2, ...] → AggregatedPubKey
2. Nonce Generation:  Each party generates random nonces
3. Partial Signing:   Each party creates partial signature
4. Aggregation:       Combine partial signatures → Final signature
5. Broadcast:         Send signed transaction to Solana
```

### 4. **Token Operations (`token.rs`)**

**Purpose**: Handles SPL token-specific operations on Solana.

**Key Components**:
- `create_token_mint()`: Creates new SPL token mints
- `mint_tokens_to()`: Mints tokens to a destination wallet
- `transfer_tokens()`: Transfers tokens between wallets
- `get_token_balance()`: Queries token balances

**Integration Points**:
- Works with TSS module for MPC-controlled operations
- Handles Associated Token Account (ATA) creation
- Manages Solana transaction construction

### 5. **Serialization (`serialization.rs`)**

**Purpose**: Handles serialization and deserialization of MPC protocol messages.

**Key Components**:
- `Serialize` trait: Common interface for serialization
- `AggMessage1`: First message in MPC protocol (nonces)
- `SecretAggStepOne`: Secret state after first step
- `PartialSignature`: Partial signature wrapper
- Base58 encoding/decoding for command-line usage

**Message Types**:
```
Tag::AggMessage1     → Public nonces from step one
Tag::PartialSignature → Partial signatures from step two  
Tag::SecretAggStepOne → Private state after step one
```

### 6. **Error Handling (`error.rs`)**

**Purpose**: Centralized error management across all modules.

**Error Categories**:
- **Network Errors**: Invalid network specifications
- **Cryptographic Errors**: Key parsing, signature failures
- **Blockchain Errors**: RPC failures, transaction errors
- **Protocol Errors**: MPC protocol violations
- **Token Errors**: SPL token operation failures

## 🔄 Data Flow

### Standard Token Operation Flow

```
1. User Input → CLI Parsing → Command Validation
2. Network Connection → RPC Client Creation  
3. Key Parsing → Keypair Objects
4. Token Operation → SPL Instructions
5. Transaction Creation → Signing → Broadcast
```

### MPC Token Operation Flow

```
1. Step One (Per Party):
   User Key → Nonce Generation → Public Message + Secret State

2. Message Exchange:
   Parties share Public Messages off-chain

3. Step Two (Per Party):
   Secret State + All Public Messages + Transaction Details 
   → Partial Signature

4. Signature Aggregation:
   All Partial Signatures → Final Signature

5. Transaction Broadcast:
   Final Signature + Transaction → Solana Network
```

## 🔐 Security Architecture

### Key Security Principles

1. **Private Key Isolation**: Private keys never leave their origin party
2. **Nonce Security**: Fresh random nonces for each signing session
3. **Message Authenticity**: All MPC messages are cryptographically bound
4. **Signature Verification**: Final transactions are verified before broadcast

### Trust Model

- **No Trusted Third Party**: The protocol doesn't require a trusted coordinator
- **Threshold Security**: All parties must participate (n-of-n currently)
- **Cryptographic Guarantees**: Security relies on elliptic curve cryptography

## 🌐 Network Integration

### Solana Integration Points

1. **RPC Client**: Communicates with Solana validators
2. **Transaction Construction**: Builds Solana-compatible transactions
3. **Account Management**: Handles Associated Token Accounts
4. **Fee Management**: Calculates and includes appropriate fees

### Supported Networks

- **Mainnet**: Production Solana network
- **Testnet**: Stable testing network
- **Devnet**: Development and testing network  
- **Localnet**: Local validator for development

## 📊 Performance Considerations

### Computational Complexity

- **Key Aggregation**: O(n) where n = number of parties
- **Signing**: O(n) per party, O(n²) total communication
- **Verification**: O(1) - same as single signature verification

### Network Requirements

- **Round Trips**: 2 communication rounds for signing
- **Message Size**: ~200 bytes per party per round
- **Latency**: Dependent on off-chain communication method

## 🔮 Future Extensions

The architecture is designed to support:

- **Threshold Signatures**: k-of-n instead of n-of-n
- **Additional Protocols**: Other MPC signature schemes
- **Batch Operations**: Multiple token operations in one MPC session
- **Cross-Chain Support**: Extension to other blockchains

---

**Next**: Learn about the [MPC Protocol](./mpc-protocol.md) in detail or explore the [API Reference](./api-reference.md). 