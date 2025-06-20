#!/bin/bash

set -e  # Exit on any error

echo "=== MPC Token Transfer Test Script ==="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_step() {
    echo -e "${BLUE}=== $1 ===${NC}"
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

# Step 1: Generate 3 keypairs for MPC participants
print_step "Step 1: Generating 3 keypairs for MPC participants"

echo "Generating keypair 1..."
output1=$(cargo run -- generate 2>/dev/null)
pubkey1=$(echo "$output1" | grep "public key:" | cut -d' ' -f3)
privkey1=$(echo "$output1" | grep "secret share (base58):" | cut -d' ' -f4)

echo "Generating keypair 2..."
output2=$(cargo run -- generate 2>/dev/null)
pubkey2=$(echo "$output2" | grep "public key:" | cut -d' ' -f3)
privkey2=$(echo "$output2" | grep "secret share (base58):" | cut -d' ' -f4)

echo "Generating keypair 3..."
output3=$(cargo run -- generate 2>/dev/null)
pubkey3=$(echo "$output3" | grep "public key:" | cut -d' ' -f3)
privkey3=$(echo "$output3" | grep "secret share (base58):" | cut -d' ' -f4)

print_success "Generated 3 keypairs"
print_info "Pubkey 1: $pubkey1"
print_info "Pubkey 2: $pubkey2"
print_info "Pubkey 3: $pubkey3"
echo

# Step 2: Generate aggregated key
print_step "Step 2: Generating aggregated key"

agg_output=$(cargo run -- aggregate-keys "$pubkey1" "$pubkey2" "$pubkey3" 2>/dev/null)
agg_key=$(echo "$agg_output" | grep "The Aggregated Public Key:" | cut -d' ' -f5)

print_success "Aggregated key generated: $agg_key"
echo

# Step 3: Generate token creator keypair and create token
print_step "Step 3: Creating token and minting initial supply"

echo "Generating token creator keypair..."
creator_output=$(cargo run -- generate 2>/dev/null)
creator_pubkey=$(echo "$creator_output" | grep "public key:" | cut -d' ' -f3)
creator_privkey=$(echo "$creator_output" | grep "secret share (base58):" | cut -d' ' -f4)

print_info "Creator pubkey: $creator_pubkey"

# Airdrop SOL for transaction fees
echo "Airdropping SOL for transaction fees..."
cargo run -- airdrop --to "$creator_pubkey" --amount 2 --net localnet >/dev/null 2>&1
cargo run -- airdrop --to "$agg_key" --amount 1 --net localnet >/dev/null 2>&1
print_success "Airdropped SOL to creator and aggregated key"

echo "Creating token..."
token_output=$(cargo run -- create-token --mint-authority-key "$creator_privkey" --decimals 6 --net localnet 2>/dev/null)
mint_address=$(echo "$token_output" | grep "Mint address:" | awk '{print $3}')

print_success "Token created with mint address: $mint_address"

echo "Minting 50 tokens to creator..."
cargo run -- mint-tokens --mint "$mint_address" --mint-authority-key "$creator_privkey" --to "$creator_pubkey" --amount 50 --decimals 6 --net localnet >/dev/null 2>&1

echo "Minting 50 tokens to aggregated key..."
cargo run -- mint-tokens --mint "$mint_address" --mint-authority-key "$creator_privkey" --to "$agg_key" --amount 50 --decimals 6 --net localnet >/dev/null 2>&1

print_success "Minted tokens to both creator and aggregated key"
echo

# Step 4: Check balances
print_step "Step 4: Checking token balances"

creator_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$creator_pubkey" --net localnet 2>/dev/null | grep -o '[0-9]\+ tokens')
agg_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$agg_key" --net localnet 2>/dev/null | grep -o '[0-9]\+ tokens')

print_info "Creator balance: $creator_balance"
print_info "Aggregated key balance: $agg_balance"
echo

# Step 5: MPC Step 1 - Generate nonces
print_step "Step 5: MPC Step 1 - Generating nonces"

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
echo

# Step 6: Get recent blockhash and perform MPC steps quickly
print_step "Step 6: MPC Steps 2 & 3 - Signature generation and aggregation"

echo "Getting recent blockhash..."
recent_blockhash=$(cargo run -- recent-block-hash --net localnet 2>/dev/null | grep "Recent blockhash:" | cut -d' ' -f3)
print_info "Recent blockhash: $recent_blockhash"

echo "Performing MPC Step 2 for all participants..."

# Participant 1 (excludes their own public share)
print_info "Generating signature from participant 1..."
sig_output1=$(cargo run -- agg-send-step-two-token \
  --private-key "$privkey1" \
  --mint "$mint_address" \
  --amount 25 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public_share2,$public_share3" \
  --secret-state "$secret_share1" \
  --net localnet 2>/dev/null)
signature1=$(echo "$sig_output1" | grep "partial signature:" | cut -d' ' -f3)

# Participant 2 (excludes their own public share)
print_info "Generating signature from participant 2..."
sig_output2=$(cargo run -- agg-send-step-two-token \
  --private-key "$privkey2" \
  --mint "$mint_address" \
  --amount 25 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public_share1,$public_share3" \
  --secret-state "$secret_share2" \
  --net localnet 2>/dev/null)
signature2=$(echo "$sig_output2" | grep "partial signature:" | cut -d' ' -f3)

# Participant 3 (excludes their own public share)
print_info "Generating signature from participant 3..."
sig_output3=$(cargo run -- agg-send-step-two-token \
  --private-key "$privkey3" \
  --mint "$mint_address" \
  --amount 25 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --first-messages "$public_share1,$public_share2" \
  --secret-state "$secret_share3" \
  --net localnet 2>/dev/null)
signature3=$(echo "$sig_output3" | grep "partial signature:" | cut -d' ' -f3)

print_success "Generated partial signatures from all participants"
print_info "Signature 1: $signature1"
print_info "Signature 2: $signature2"
print_info "Signature 3: $signature3"

# Step 7: Aggregate signatures and broadcast
echo "Performing MPC Step 3 - Aggregating signatures and broadcasting..."
final_output=$(cargo run -- aggregate-signatures-and-broadcast-token \
  --signatures "$signature1,$signature2,$signature3" \
  --mint "$mint_address" \
  --amount 25 \
  --decimals 6 \
  --to "$creator_pubkey" \
  --recent-block-hash "$recent_blockhash" \
  --keys "$pubkey1,$pubkey2,$pubkey3" \
  --net localnet 2>/dev/null)

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

# Step 8: Verify final balances
print_step "Step 8: Verifying final balances"

final_creator_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$creator_pubkey" --net localnet 2>/dev/null | grep -o '[0-9]\+ tokens')
final_agg_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$agg_key" --net localnet 2>/dev/null | grep -o '[0-9]\+ tokens')

print_info "Final creator balance: $final_creator_balance"
print_info "Final aggregated key balance: $final_agg_balance"

echo
print_success "MPC Token Transfer Test Completed Successfully! 🎉"
echo -e "${GREEN}Summary:${NC}"
echo -e "${GREEN}  • Created 3 MPC participant keypairs${NC}"
echo -e "${GREEN}  • Generated aggregated key: $agg_key${NC}"
echo -e "${GREEN}  • Created token: $mint_address${NC}"
echo -e "${GREEN}  • Minted 50 tokens each to creator and aggregated key${NC}"
echo -e "${GREEN}  • Transferred 25 tokens using MPC from aggregated key back to creator${NC}"
echo -e "${GREEN}  • Transaction completed: $transaction_id${NC}" 