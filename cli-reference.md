# CLI Reference

This document provides a comprehensive reference for all CLI commands available in the Solana MPC Tokens project. Each command includes its syntax, parameters, and example output.

## Key Management Commands

### Generate New Keypair
Generates a new keypair for MPC participation.

```bash
cargo run generate
```

**Output:**
```
secret share (base58): 38XAevK3MC3EfRy7...W7B1
public key: SBThUTrhoVxhDP8HS6KRGJner4fQGEksUFLMAP81X23
```

### Aggregate Public Keys
Creates an aggregated public key from multiple participant keys.

```bash
cargo run -- aggregate-keys <PUBKEY1> <PUBKEY2> <PUBKEY3>
```

**Output:**
```
The Aggregated Public Key: PiRVVAEZEJns51Zv6pDfM3YyD9UEUdmDQaqFBDJ9pHC
```

## Account Management Commands

### Airdrop SOL
Request an airdrop of SOL to a specified account.

```bash
cargo run -- airdrop --to <PUBKEY> --amount <SOL_AMOUNT> --net localnet
```

**Output:**
```
Airdrop transaction ID: 4KxUjyeUh9WehoRZxEdC11nn2CA93dimEK28H8Dd7axB1UdR4t98FFxuUZGmrmPNsUb1Z3j6fbcfKxzKv2sojGBt
```

### Check SOL Balance
View the SOL balance of an account.

```bash
cargo run -- balance <PUBKEY> --net localnet
```

**Output:**
```
The balance of SBThUTrhoVxhDP8HS6KRGJner4fQGEksUFLMAP81X23 is: 2000000000
```

### Get Recent Blockhash
Retrieve the recent blockhash (required for transactions).

```bash
cargo run recent-block-hash
```

**Output:**
```
Recent blockhash: HAUQZ1jHUxCoUc6zNEHnrKFAbxyzJ9VZhxsGF92nu9HH
```

## Token Commands

### Create Token
Create a new SPL token.

```bash
cargo run create-token --mint-authority-key <PRIVATE_KEY> --decimals 6
```

**Output:**
```
Token mint created successfully!
Mint address: 4jaRnpvVu5tLYuxhGWHaoFh29zSpc4fscBEPtgGiZfcy
Transaction signature: 3yN7wVwuV1TUDz75Pbfxu9PUj3BiE51QG748B1znJrTFPGDqjRVhH2QaJqYqjqkpnhXknZSWjH11vc7RuFiay48R
```

### Mint Tokens
Mint new tokens to a specified address.

```bash
cargo run -- mint-tokens \
    --mint <MINT_ADDRESS> \
    --mint-authority-key <MINT_AUTHORITY> \
    --to <RECEIVER'S KEY> \
    --amount <amount> \
    --decimals 6 \
    --net localnet
```

**Output:**
```
Tokens minted successfully!
Mint: 4jaRnpvVu5tLYuxhGWHaoFh29zSpc4fscBEPtgGiZfcy
To: 7E8azFTTKghsRfGutBVAjHdCzkV3ZncXuKYvVvzdmNpx
Amount: 50 tokens
Transaction signature: ZBFUP7GkTftzGoyi5Q5Mt1a6ZB1vczymoGZjrEk9v4SQMXAYAzYscGdjUA4GqnGDNG93U8kV7vrTKRur7ScnE6z
```

### Check Token Balance
View the token balance of an account.

```bash
cargo run -- token-balance --mint <MINT_ADDRESS> --wallet <WALLET_ADDRESS> --net localnet
```

**Output:**
```
Token balance for wallet 7E8azFTTKghsRfGutBVAjHdCzkV3ZncXuKYvVvzdmNpx: 100 tokens
```

## MPC Transaction Commands

### 1. Generate Nonces (Step One)
Generate nonces for each participant. Must be done by all participants.

```bash
cargo run agg-send-step-one <PRIVATE_KEY>
```

**Output:**
```
secret share: 2SgXkBtDWzAGwawwCV8PjzyvDdhh8oavQ33JndD5stKVnDG7ap5mr3ADoxX8yJuumbTfYRY1pNmSnbXY6fD6zrkgJWDHoTVvJrjGvPgvGSryaFoqSqbTrtDY68Vio96BNyEMULtoR5LQErpUum8HFsjuKbJKcHfrkvSHWqQvgfNkmzQ2
public share: 2imCcHfLaUCkQpoE9CDsCcUXqwu7NPrp2rn3uZCYqLScFD3hpc5pb2MWyenpjf3EjArc2pAgYm9mM7V7c81qNGPncvwdVKXkDsCh8vPwrhFDJe4RSJPiX6BHyz2BJjPoEZTs
```

### 2. Create Partial Signatures (Step Two)
Generate partial signatures for the transaction. Must be done by all participants.

```bash
cargo run -- agg-send-step-two-token \
    --private-key <base58_private_key> \
    --mint <token_mint_address> \
    --amount <amount> \
    --decimals 6 \
    --to <recipient_pubkey> \
    --recent-block-hash <hash_from_network> \
    --keys <pubkey1,pubkey2,pubkey3> \
    --first-messages <msg1_from_step1,msg2_from_step1,msg3_from_step1> \
    --secret-state <secret_from_step1> \
    --net localnet
```

**Output:**
```
partial signature: EEWSmnxp9unKdt1yosyjMMAXvcjUgekz6mH2Yr63v849ugh21DuXqMz3LeYitoqEi17FVXB7tLsCBYWAMxw4xt3a
```

### 3. Aggregate and Broadcast (Final Step)
Combine partial signatures and broadcast the transaction.

```bash
cargo run -- aggregate-signatures-and-broadcast-token \
    --signatures <signature1,signature2,signature3> \
    --mint <mint_address> \
    --amount <amount> \
    --decimals 6 \
    --to <recipient_pubkey> \
    --recent-block-hash <recent_blockhash> \
    --keys <pubkey1,pubkey2,pubkey3> \
    --net localnet
```

**Output:**
```
✓ MPC Token Transfer Test Completed Successfully! 🎉
Summary:
• Used existing aggregated key: pC5rYzWKfkMs2uM6FeQojRzCD2k8Zz8dB9BesAmDZn3
• Used existing token: 4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
• Transferred 25 tokens using MPC from aggregated key to creator
• Transaction completed: 3yiz6FFnRFk5TCriLkFgZRs8PVSo1ZSbZZxrvX7pfbjfxwzJtQ3vzqjNvM8FZWnj21dTQ63ocqr6UzXqBXze55GA
```

## Important Notes

1. The same blockhash must be used for both Step 2 (partial signatures) and the final step (aggregation and broadcast).
2. All participants must complete Step 1 (nonce generation) and Step 2 (partial signatures) before proceeding to the final step.
3. The order of public keys in the `--keys` parameter must be consistent across all commands in the MPC process. 