#!/bin/bash

# Simple SOL MPC Transfer Test using existing aggregated wallet
echo "=== SOL MPC Transfer Test ==="

# Use the existing manually tested values
pubkey1="5s78GKriutGNSPDxP8mDK95bo2mx7mvchK4jdEg7SyFY"
pubkey2="9eyymZqK6SdUhKwvNz93kFHWUAbmB6n1dEZ3rXwT22JM"
pubkey3="E4qnX4EW4Cb8xp4vzPis8teRJR4rSkCiVQ6MLaZniCmT"
privkey1="4Qb4Sy1uD69zpo3cYZa4TFu7ZHMT8gKEr2S8rRYeiZAVJG2Q9F4YijwQgpS9KwtkAVNosUNknX2JS7NzPiHBHCKL"
privkey2="5xBJpyznDiXq5AUNQqe3N5v6NpbfGLEJwstB4orNRXAubq2bcCiFF4CyHbKThYgb1dmS1iQ1hXJXvBLdjpJ4M8JM"
privkey3="EeQKUt9TezZaWwwoBaBkBD1e2dDb9m3k9yR6janDhtHHAb19HJGs4JfnjsJKSn712zguZEhZaLcBgKBCgio8vB7"
agg_key="pC5rYzWKfkMs2uM6FeQojRzCD2k8Zz8dB9BesAmDZn3"
creator_pubkey="HUpF1G1kRfd1nomJv4XsYGxPVvQ662T5zWni2favw7CV"

echo "Sending 0.01 SOL from $agg_key to $creator_pubkey"
echo

# Step 1: Check SOL balances
echo "Current SOL balances:"
agg_balance=$(cargo run -- balance $agg_key --net devnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
creator_balance=$(cargo run -- balance $creator_pubkey --net devnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
agg_sol=$(echo "scale=3; $agg_balance / 1000000000" | bc -l)
creator_sol=$(echo "scale=3; $creator_balance / 1000000000" | bc -l)
echo "Aggregated wallet: $agg_sol SOL ($agg_balance lamports)"
echo "Creator wallet: $creator_sol SOL ($creator_balance lamports)"
echo

# Step 2: Generate nonces (MPC Step 1)
echo "Generating nonces..."
nonce1=$(cargo run -- agg-send-step-one "$privkey1" 2>/dev/null)
secret1=$(echo "$nonce1" | grep "secret share:" | cut -d' ' -f3)
public1=$(echo "$nonce1" | grep "public share:" | cut -d' ' -f3)

nonce2=$(cargo run -- agg-send-step-one "$privkey2" 2>/dev/null)
secret2=$(echo "$nonce2" | grep "secret share:" | cut -d' ' -f3)
public2=$(echo "$nonce2" | grep "public share:" | cut -d' ' -f3)

nonce3=$(cargo run -- agg-send-step-one "$privkey3" 2>/dev/null)
secret3=$(echo "$nonce3" | grep "secret share:" | cut -d' ' -f3)
public3=$(echo "$nonce3" | grep "public share:" | cut -d' ' -f3)

echo "✓ Nonces generated"

# Step 3: Get recent blockhash
echo "Getting recent blockhash..."
recent_blockhash=$(cargo run -- recent-block-hash --net devnet 2>/dev/null | grep "Recent blockhash:" | cut -d' ' -f3)
echo "✓ Blockhash: $recent_blockhash"

# Step 4: Generate partial signatures (MPC Step 2)
echo "Generating partial signatures..."

sig1=$(cargo run -- agg-send-step-two-sol \
  --private-key "$privkey1" \
  --amount 0.01 \
  --to "$creator_pubkey" \
  --memo "MPC SOL Test" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public2,$public3" \
  --secret-state "$secret1" \
  --net devnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)

sig2=$(cargo run -- agg-send-step-two-sol \
  --private-key "$privkey2" \
  --amount 0.01 \
  --to "$creator_pubkey" \
  --memo "MPC SOL Test" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public1,$public3" \
  --secret-state "$secret2" \
  --net devnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)

sig3=$(cargo run -- agg-send-step-two-sol \
  --private-key "$privkey3" \
  --amount 0.01 \
  --to "$creator_pubkey" \
  --memo "MPC SOL Test" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public1,$public2" \
  --secret-state "$secret3" \
  --net devnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)

echo "✓ Partial signatures generated"

# Step 5: Aggregate and broadcast (MPC Step 3)
echo "Aggregating signatures and broadcasting..."
result=$(cargo run -- aggregate-signatures-and-broadcast-sol \
  --signatures "$sig1,$sig2,$sig3" \
  --amount 0.01 \
  --to "$creator_pubkey" \
  --memo "MPC SOL Test" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --net devnet 2>/dev/null)

if [[ $result == *"Transaction ID"* ]]; then
    tx_id=$(echo "$result" | grep "Transaction ID:" | cut -d' ' -f3)
    echo "✓ SOL transfer successful!"
    echo "Transaction ID: $tx_id"
    echo
    
    # Check final balances
    echo "Waiting for transaction to be processed..."
    sleep 15
    
    echo "Checking balances after transaction..."
    final_agg=$(cargo run -- balance $agg_key --net devnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
    final_creator=$(cargo run -- balance $creator_pubkey --net devnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
    final_agg_sol=$(echo "scale=3; $final_agg / 1000000000" | bc -l)
    final_creator_sol=$(echo "scale=3; $final_creator / 1000000000" | bc -l)
    
    # If balance didn't change, wait a bit more and try again
    if [ "$final_creator" = "$creator_balance" ]; then
        echo "Balance unchanged, waiting 10 more seconds..."
        sleep 10
        final_agg=$(cargo run -- balance $agg_key --net devnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
        final_creator=$(cargo run -- balance $creator_pubkey --net devnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
        final_agg_sol=$(echo "scale=3; $final_agg / 1000000000" | bc -l)
        final_creator_sol=$(echo "scale=3; $final_creator / 1000000000" | bc -l)
    fi
    
    echo "Final SOL balances:"
    echo "Aggregated wallet: $final_agg_sol SOL ($final_agg lamports)"
    echo "Creator wallet: $final_creator_sol SOL ($final_creator lamports)"
    echo
    echo "🎉 SOL MPC transfer completed successfully!"
else
    echo "❌ Transfer failed:"
    echo "$result"
fi 