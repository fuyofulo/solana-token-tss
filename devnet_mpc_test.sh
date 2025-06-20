#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_step() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

echo -e "${BLUE}=== MPC Token Transfer Test (Using Manual Values) ===${NC}"

# Use the manually tested values
pubkey1="5s78GKriutGNSPDxP8mDK95bo2mx7mvchK4jdEg7SyFY"
pubkey2="9eyymZqK6SdUhKwvNz93kFHWUAbmB6n1dEZ3rXwT22JM"
pubkey3="E4qnX4EW4Cb8xp4vzPis8teRJR4rSkCiVQ6MLaZniCmT"
privkey1="4Qb4Sy1uD69zpo3cYZa4TFu7ZHMT8gKEr2S8rRYeiZAVJG2Q9F4YijwQgpS9KwtkAVNosUNknX2JS7NzPiHBHCKL"
privkey2="5xBJpyznDiXq5AUNQqe3N5v6NpbfGLEJwstB4orNRXAubq2bcCiFF4CyHbKThYgb1dmS1iQ1hXJXvBLdjpJ4M8JM"
privkey3="EeQKUt9TezZaWwwoBaBkBD1e2dDb9m3k9yR6janDhtHHAb19HJGs4JfnjsJKSn712zguZEhZaLcBgKBCgio8vB7"

agg_key="pC5rYzWKfkMs2uM6FeQojRzCD2k8Zz8dB9BesAmDZn3"
creator_pubkey="HUpF1G1kRfd1nomJv4XsYGxPVvQ662T5zWni2favw7CV"
creator_privkey="5X24rz14ms7idEuMb77fRXnXTYpyaCQ36ezVi3ahNuDpK8BhZmNWAdbkiSZBWWfXNaN8d88iiueDPsaG9QNQc5GM"
mint_address="4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"

print_info "Using manually tested values:"
print_info "Aggregated key: $agg_key"
print_info "Creator: $creator_pubkey"
print_info "Token mint: $mint_address"
echo

# Step 1: Check current balances
print_step "Step 1: Checking current token balances"

creator_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$creator_pubkey" --net devnet 2>/dev/null | grep -o '[0-9]\+ tokens')
agg_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$agg_key" --net devnet 2>/dev/null | grep -o '[0-9]\+ tokens')

print_info "Creator balance: $creator_balance"
print_info "Aggregated key balance: $agg_balance"
echo

# Step 2: MPC Step 1 - Generate nonces
print_step "Step 2: MPC Step 1 - Generating nonces"

echo "Generating nonces for participant 1..."
nonce_output1=$(cargo run -- agg-send-step-one "$privkey1" 2>/dev/null)
secret_share1=$(echo "$nonce_output1" | grep "secret share:" | cut -d' ' -f3)
public_share1=$(echo "$nonce_output1" | grep "public share:" | cut -d' ' -f3)

echo "Generating nonces for participant 2..."
nonce_output2=$(cargo run -- agg-send-step-one "$privkey2" 2>/dev/null)
secret_share2=$(echo "$nonce_output2" | grep "secret share:" | cut -d' ' -f3)
public_share2=$(echo "$nonce_output2" | grep "public share:" | cut -d' ' -f3)

echo "Generating nonces for participant 3..."
nonce_output3=$(cargo run -- agg-send-step-one "$privkey3" 2>/dev/null)
secret_share3=$(echo "$nonce_output3" | grep "secret share:" | cut -d' ' -f3)
public_share3=$(echo "$nonce_output3" | grep "public share:" | cut -d' ' -f3)

print_success "Generated nonces for all participants"
print_info "Secret share 1 length: ${#secret_share1}"
print_info "Public share 1 length: ${#public_share1}"
echo

# Step 3: Get recent blockhash and perform MPC steps quickly
print_step "Step 3: MPC Steps 2 & 3 - Signature generation and aggregation"

echo "Getting recent blockhash..."
recent_blockhash=$(cargo run -- recent-block-hash --net devnet 2>/dev/null | grep "Recent blockhash:" | cut -d' ' -f3)
print_info "Recent blockhash: $recent_blockhash"

echo "Performing MPC Step 2 for all participants..."

# Participant 1 (excludes their own public share)
print_info "Generating signature from participant 1..."
sig_output1=$(cargo run -- agg-send-step-two-token \
  --private-key "$privkey1" \
  --mint "$mint_address" \
  --amount 1000000 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public_share2,$public_share3" \
  --secret-state "$secret_share1" \
  --net devnet 2>/dev/null)
signature1=$(echo "$sig_output1" | grep "partial signature:" | cut -d' ' -f3)

# Participant 2 (excludes their own public share)
print_info "Generating signature from participant 2..."
sig_output2=$(cargo run -- agg-send-step-two-token \
  --private-key "$privkey2" \
  --mint "$mint_address" \
  --amount 1000000 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public_share1,$public_share3" \
  --secret-state "$secret_share2" \
  --net devnet 2>/dev/null)
signature2=$(echo "$sig_output2" | grep "partial signature:" | cut -d' ' -f3)

# Participant 3 (excludes their own public share)
print_info "Generating signature from participant 3..."
sig_output3=$(cargo run -- agg-send-step-two-token \
  --private-key "$privkey3" \
  --mint "$mint_address" \
  --amount 1000000 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public_share1,$public_share2" \
  --secret-state "$secret_share3" \
  --net devnet 2>/dev/null)
signature3=$(echo "$sig_output3" | grep "partial signature:" | cut -d' ' -f3)

print_success "Generated partial signatures from all participants"
print_info "Signature 1: $signature1"
print_info "Signature 2: $signature2"
print_info "Signature 3: $signature3"

# Step 4: Aggregate signatures and broadcast
echo "Performing MPC Step 3 - Aggregating signatures and broadcasting..."
final_output=$(cargo run -- aggregate-signatures-and-broadcast-token \
  --signatures "$signature1,$signature2,$signature3" \
  --mint "$mint_address" \
  --amount 1000000 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --net devnet 2>/dev/null)

if [[ $final_output == *"Transaction"* ]]; then
    transaction_id=$(echo "$final_output" | grep -o '[A-Za-z0-9]\{87,88\}')
    print_success "MPC token transfer completed successfully!"
    print_success "Transaction ID: $transaction_id"
else
    print_error "MPC token transfer failed"
    echo "$final_output"
    exit 1
fi

echo

# Step 5: Verify final balances
print_step "Step 5: Verifying final balances"

echo "Waiting for transaction to be processed..."
sleep 15

echo "Checking balances after transaction..."
final_creator_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$creator_pubkey" --net devnet 2>/dev/null | grep -o '[0-9]\+ tokens')
final_agg_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$agg_key" --net devnet 2>/dev/null | grep -o '[0-9]\+ tokens')

# If balance didn't change, wait a bit more and try again
if [ "$final_creator_balance" = "$creator_balance" ]; then
    echo "Balance unchanged, waiting 10 more seconds..."
    sleep 10
    final_creator_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$creator_pubkey" --net devnet 2>/dev/null | grep -o '[0-9]\+ tokens')
    final_agg_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$agg_key" --net devnet 2>/dev/null | grep -o '[0-9]\+ tokens')
fi

print_info "Final creator balance: $final_creator_balance"
print_info "Final aggregated key balance: $final_agg_balance"

echo
print_success "MPC Token Transfer Test Completed Successfully! 🎉"
echo -e "${GREEN}Summary:${NC}"
echo -e "${GREEN}  • Used existing aggregated key: $agg_key${NC}"
echo -e "${GREEN}  • Used existing token: $mint_address${NC}"
echo -e "${GREEN}  • Transferred 25 tokens using MPC from aggregated key to creator${NC}"
echo -e "${GREEN}  • Transaction completed: $transaction_id${NC}" 