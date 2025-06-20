# Solana MPC Tokens Documentation

Welcome to the comprehensive documentation for **Solana MPC Tokens**, a cutting-edge implementation of Multi-Party Computation (MPC) for SPL token operations on the Solana blockchain.

## What is Solana MPC Tokens?

Solana MPC Tokens is a command-line application that enables **multi-party signatures** for SPL token operations using **MuSig2 protocol**. Instead of a single private key controlling tokens, multiple parties can collectively sign transactions without ever revealing individual private keys or reconstructing the full private key.

## 🔑 Key Features

- **Multi-Party Token Control**: Multiple parties jointly control SPL tokens
- **Multi-Party Signatures**: No single point of failure - all parties must participate
- **Privacy Preserving**: Individual private keys never leave their owners
- **Solana Native**: Built specifically for SPL tokens on Solana
- **MuSig2 Protocol**: State-of-the-art cryptographic security

## 📚 Documentation Structure

This documentation is organized into the following sections:

### 🏗️ Architecture & Design
- **[Architecture Overview](./architecture.md)** - High-level system design and module interactions
- **[MPC Protocol Guide](./mpc-protocol.md)** - Deep dive into the MuSig2 implementation
- **[Error Handling](./error-handling.md)** - How errors are managed throughout the system

### 🔧 Technical Reference
- **[API Reference](./api-reference.md)** - Complete function and module documentation
- **[CLI Commands](./cli-reference.md)** - Detailed command-line interface guide
- **[Data Structures](./data-structures.md)** - Core types and serialization formats

### 📖 User Guides
- **[Getting Started](./getting-started.md)** - Quick start guide for new users
- **[Usage Examples](./usage-examples.md)** - Practical scenarios and workflows
- **[Token Operations](./token-operations.md)** - SPL token management with MPC

### 🛠️ Development
- **[Development Guide](./development-guide.md)** - For contributors and developers
- **[Testing Guide](./testing-guide.md)** - How to run and write tests
- **[Security Considerations](./security.md)** - Important security aspects and best practices

## 🚀 Quick Start

If you're new to the project, we recommend starting with:

1. **[Getting Started](./getting-started.md)** - Learn the basics and setup
2. **[Architecture Overview](./architecture.md)** - Understand how it works
3. **[Usage Examples](./usage-examples.md)** - See it in action
4. **[CLI Commands](./cli-reference.md)** - Master the command-line interface

## 🔐 What Makes This Special?

Traditional cryptocurrency wallets require a single private key to control funds. If that key is compromised or lost, all funds are at risk. Solana MPC Tokens solves this by:

1. **Distributing Trust**: Multiple parties must cooperate to sign transactions
2. **Eliminating Single Points of Failure**: No one party has complete control
3. **Maintaining Privacy**: Individual secrets never need to be shared
4. **Providing Flexibility**: Supports multiple participants in signing

## 🎯 Use Cases

- **Corporate Treasury Management**: Multiple executives must approve large transfers
- **Shared Wallet Services**: Service providers without full custody
- **Enhanced Security**: Protection against key theft or coercion
- **Compliance**: Regulatory requirements for multi-signature approvals

## 🔍 Quick Example

Here's a simple 2-party token transfer:

```bash
# Party 1: Generate MPC nonces
cargo run -- agg-send-step-one <private_key_1>

# Party 2: Generate MPC nonces  
cargo run -- agg-send-step-one <private_key_2>

# Both parties: Create partial signatures
cargo run -- agg-send-step-two-token --mint <token> --amount 100 ...

# Anyone: Aggregate and broadcast transaction
cargo run -- aggregate-signatures-and-broadcast-token --signatures <sig1,sig2> ...
```

## 📋 Prerequisites

Before diving into the documentation, you should have:

- Basic understanding of **blockchain** and **cryptocurrency** concepts
- Familiarity with **Solana** and **SPL tokens**
- Knowledge of **command-line interfaces**
- Basic **cryptography** concepts (helpful but not required)

## 🤝 Contributing

Found an issue with the documentation? Want to improve it? Please see our **[Development Guide](./development-guide.md)** for contribution guidelines.

---

**Next Steps**: Start with [Getting Started](./getting-started.md) or jump to [Architecture Overview](./architecture.md) for a technical deep-dive. 