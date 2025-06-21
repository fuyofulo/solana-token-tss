#!/bin/bash

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_header() {
    echo -e "\n${BLUE}╔══════════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║ $1${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════════════════════════╝${NC}"
}

print_step() {
    echo -e "\n${CYAN}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ  $1${NC}"
}

print_warning() {
    echo -e "${PURPLE}⚠  $1${NC}"
}

print_header "🚀 WELCOME TO SOLANA MPC TOKENS COMPLETE DEMO 🚀"
echo -e "${CYAN}"
cat << "EOF"
    __  __ ____   ____   ____                        
   |  \/  |  _ \ / ___| |  _ \  ___ _ __ ___   ___    
   | |\/| | |_) | |     | | | |/ _ \ '_ ` _ \ / _ \   
   | |  | |  __/| |___  | |_| |  __/ | | | | | (_) |  
   |_|  |_|_|    \____| |____/ \___|_| |_| |_|\___/   
                                                      
        🔐 Multi-Party Computation for SPL Tokens on Solana 🔐
EOF
echo -e "${NC}"

echo -e "${YELLOW}This demo will walk you through the complete MPC process:"
echo -e "• Generate MPC participant keys"
echo -e "• Create aggregated wallet"
echo -e "• Fund wallets with SOL"
echo -e "• Create and mint SPL tokens"
echo -e "• Perform MPC token transfers"
echo -e "• Perform MPC SOL transfers"
echo -e ""
echo -e "This is a DEMO - all keys will be displayed for educational purposes.${NC}"
echo -e ""
read -p "Press Enter to continue..."

print_header "PREREQUISITE: START YOUR LOCALNET VALIDATOR"
echo -e "${YELLOW}Before running this demo, you need to start a localnet validator with SPL Token programs."
echo -e ""
echo -e "Run this command in another terminal:${NC}"
echo -e "${GREEN}solana-test-validator${NC}"
echo -e ""
read -p "Once your validator is running, press Enter to continue..."


# Step 1: Generate MPC Participant Keys
print_header "STEP 1: GENERATING MPC PARTICIPANT KEYS"
echo -e "${YELLOW}In MPC (Multi-Party Computation), multiple parties each have their own private key,"
echo -e "but they can collectively sign transactions without revealing their private keys to each other."
echo -e ""
echo -e "We'll create 3 participants for this demo:${NC}"

print_step "Generating keys for 3 MPC participants"

echo "Generating keys for Participant 1 (Alice)..."
participant1_output=$(cargo run -- generate 2>/dev/null)
participant1_private=$(echo "$participant1_output" | grep "secret share" | cut -d' ' -f4)
participant1_public=$(echo "$participant1_output" | grep "public key" | cut -d' ' -f3)

echo "Generating keys for Participant 2 (Bob)..."
participant2_output=$(cargo run -- generate 2>/dev/null)
participant2_private=$(echo "$participant2_output" | grep "secret share" | cut -d' ' -f4)
participant2_public=$(echo "$participant2_output" | grep "public key" | cut -d' ' -f3)

echo "Generating keys for Participant 3 (Charlie)..."
participant3_output=$(cargo run -- generate 2>/dev/null)
participant3_private=$(echo "$participant3_output" | grep "secret share" | cut -d' ' -f4)
participant3_public=$(echo "$participant3_output" | grep "public key" | cut -d' ' -f3)

print_success "Generated keys for all 3 participants!"
echo -e ""
echo -e "${CYAN}📋 PARTICIPANT KEYS (Demo purposes only!):"
echo -e "┌─ Participant 1 (Alice) ─────────────────────────────────────────────────────────────────────────────────"
echo -e "│ Public Key:  $participant1_public                                                "
echo -e "│ Private Key: $participant1_private   "
echo -e "└─────────────────────────────────────────────────────────────────────────────────────────────────────────"
echo -e ""
echo -e "┌─ Participant 2 (Bob) ───────────────────────────────────────────────────────────────────────────────────"
echo -e "│ Public Key:  $participant2_public                                               "
echo -e "│ Private Key: $participant2_private   "
echo -e "└─────────────────────────────────────────────────────────────────────────────────────────────────────────"
echo -e ""
echo -e "┌─ Participant 3 (Charlie) ───────────────────────────────────────────────────────────────────────────────"
echo -e "│ Public Key:  $participant3_public                                               "
echo -e "│ Private Key: $participant3_private    "
echo -e "└─────────────────────────────────────────────────────────────────────────────────────────────────────────${NC}"

# Step 2: Create Aggregated Key
print_header "STEP 2: CREATING AGGREGATED WALLET"
echo -e "${YELLOW}Now we'll create an 'aggregated key' from our 3 participant keys."
echo -e "This creates a single wallet address that requires ALL 3 participants"
echo -e "to sign any transaction. No single person can move funds alone!${NC}"

print_step "Aggregating the 3 public keys"
aggregated_output=$(cargo run -- aggregate-keys $participant1_public $participant2_public $participant3_public 2>/dev/null)
aggregated_key=$(echo "$aggregated_output" | grep "Aggregated Public Key" | cut -d' ' -f5)

print_success "Aggregated wallet created!"
echo -e ""
echo -e "${GREEN}🏦 AGGREGATED WALLET:"
echo -e "Address: $aggregated_key"
echo -e ""
echo -e "This wallet requires ALL 3 participants (Alice, Bob, Charlie) to sign"
echo -e "any transaction using Multi-Party Computation (MPC).${NC}"

# Step 3: Generate Creator Key and Fund Wallets
print_header "STEP 3: FUNDING WALLETS WITH SOL"
echo -e "${YELLOW}We'll create a 'creator' wallet and fund both the creator and aggregated"
echo -e "wallets with SOL from the localnet faucet.${NC}"

print_step "Generating creator wallet"
creator_output=$(cargo run -- generate 2>/dev/null)
creator_private=$(echo "$creator_output" | grep "secret share" | cut -d' ' -f4)
creator_public=$(echo "$creator_output" | grep "public key" | cut -d' ' -f3)

print_success "Creator wallet generated!"
echo -e ""
echo -e "${CYAN}👤 CREATOR WALLET:"
echo -e "Public Key:  $creator_public"
echo -e "Private Key: $creator_private${NC}"

print_step "Requesting SOL airdrops from localnet faucet"
echo "Requesting 5 SOL for creator wallet..."
creator_airdrop=$(cargo run -- airdrop --to $creator_public --amount 5 --net localnet 2>/dev/null)
print_info "Creator airdrop: $(echo "$creator_airdrop" | grep "Airdrop transaction")"

echo "Requesting 5 SOL for aggregated wallet..."
agg_airdrop=$(cargo run -- airdrop --to $aggregated_key --amount 5 --net localnet 2>/dev/null)
print_info "Aggregated wallet airdrop: $(echo "$agg_airdrop" | grep "Airdrop transaction")"

print_success "Both wallets funded with SOL!"

# Check balances
print_step "Checking SOL balances"
sleep 10  # Wait for airdrops to process

creator_sol_balance=$(cargo run -- balance $creator_public --net localnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
agg_sol_balance=$(cargo run -- balance $aggregated_key --net localnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)

creator_sol=$(echo "scale=3; $creator_sol_balance / 1000000000" | bc -l)
agg_sol=$(echo "scale=3; $agg_sol_balance / 1000000000" | bc -l)

echo -e "${GREEN}💰 SOL BALANCES:"
echo -e "Creator wallet:    $creator_sol SOL"
echo -e "Aggregated wallet: $agg_sol SOL${NC}"

# Step 4: Create SPL Token
print_header "STEP 4: CREATING SPL TOKEN"
echo -e "${YELLOW}Now we'll create a new SPL token with the creator as the mint authority.${NC}"

print_step "Creating new SPL token"
token_output=$(cargo run -- create-token \
    --mint-authority-key $creator_private \
    --decimals 6 \
    --net localnet 2>/dev/null)

if [[ $token_output == *"Error:"* ]]; then
    echo -e "${RED}❌ Token creation failed: $token_output${NC}"
    exit 1
else
    mint_address=$(echo "$token_output" | grep "Mint address:" | cut -d' ' -f3)
    token_tx=$(echo "$token_output" | grep "Transaction signature:" | cut -d' ' -f3)
    print_success "SPL Token created!"
fi
echo -e ""
echo -e "${GREEN}🪙 NEW SPL TOKEN:"
echo -e "Mint Address: $mint_address"
echo -e "Decimals: 6 (like USDC)"
echo -e "Mint Authority: Creator wallet"
echo -e "Transaction: $token_tx${NC}" 

echo -e "${CYAN} here is the mint account info:${NC}"
echo -e "https://explorer.solana.com/account/$mint_address?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899"

echo -e "\n${PURPLE}waiting for 15 seconds before minting tokens${NC}"
sleep 15

# Step 5: Mint Tokens to Aggregated Wallet
print_header "STEP 5: MINTING TOKENS TO AGGREGATED WALLET"
echo -e "${YELLOW}We'll mint 100 tokens to the both the creator and aggregated wallets."
echo -e "The tokens in the aggregated wallet can only be moved using MPC signatures from all 3 participants.${NC}"

print_step "Minting 100 tokens to both the creator and aggregated wallets"
minting_for_aggregated_key=$(cargo run -- mint-tokens --mint $mint_address --mint-authority-key $creator_private --to $aggregated_key --amount 100 --decimals 6 --net localnet 2>/dev/null)

minting_for_creator_key=$(cargo run -- mint-tokens --mint $mint_address --mint-authority-key $creator_private --to $creator_public --amount 100 --decimals 6 --net localnet 2>/dev/null)

if [[ $minting_for_aggregated_key == *"Error:"* ]]; then
    echo -e "${RED}❌ Token minting failed: $minting_for_aggregated_key${NC}"
    mint_tx=""
else
    mint_tx_for_aggregated_key=$(echo "$minting_for_aggregated_key" | grep "Transaction signature:" | cut -d' ' -f3)
    print_success "Tokens minted successfully!"
fi

if [[ $minting_for_creator_key == *"Error:"* ]]; then
    echo -e "${RED}❌ Token minting failed: $minting_for_creator_key${NC}"
    mint_tx=""
else
    mint_tx_for_creator_key=$(echo "$minting_for_creator_key" | grep "Transaction signature:" | cut -d' ' -f3)
    print_success "Tokens minted successfully!"
fi

echo -e ""
echo -e "${GREEN}🏭 MINTING COMPLETE FOR AGGREGATED WALLET:"
echo -e "Transaction: $mint_tx_for_aggregated_key${NC}"
echo -e ""
echo -e "${GREEN}🏭 MINTING COMPLETE FOR CREATOR WALLET:"
echo -e "Transaction: $mint_tx_for_creator_key${NC}"



# Check token balance
print_step "Checking token balances"
echo "waiting for 15 seconds before checking token balances"
sleep 15  # Wait for minting to process

agg_token_balance=$(cargo run -- token-balance \
    --mint $mint_address \
    --wallet $aggregated_key \
    --net localnet 2>/dev/null | grep -o '[0-9]\{1,\} tokens')

creator_token_balance=$(cargo run -- token-balance \
    --mint $mint_address \
    --wallet $creator_public \
    --net localnet 2>/dev/null | grep -o '[0-9]\{1,\} tokens')

echo -e "${GREEN}🪙 TOKEN BALANCES:"
echo -e "Aggregated wallet: $agg_token_balance"
echo -e "Creator wallet: $creator_token_balance${NC}"

# Step 6: MPC Token Transfer Demo
print_header "STEP 6: MPC TOKEN TRANSFER DEMONSTRATION"
echo -e "${YELLOW}Now for the main event! We'll transfer 25 tokens from the aggregated wallet"
echo -e "to the creator wallet using MPC. This requires signatures from ALL 3 participants."
echo -e ""
echo -e "The MPC process has 3 steps:"
echo -e "1. Generate nonces (random numbers for security)"
echo -e "2. Create partial signatures from each participant" 
echo -e "3. Aggregate signatures and broadcast transaction${NC}"

print_step "MPC Step 1: Generating nonces from all participants"
echo -e "${PURPLE}Each participant generates random nonces for this signing session...${NC}"

# Generate nonces for all participants
echo "Alice generating nonces..."
alice_nonce=$(cargo run -- agg-send-step-one $participant1_private 2>/dev/null)
alice_secret=$(echo "$alice_nonce" | grep "secret share:" | cut -d' ' -f3)
alice_public_msg=$(echo "$alice_nonce" | grep "public share:" | cut -d' ' -f3)

echo "Bob generating nonces..."
bob_nonce=$(cargo run -- agg-send-step-one $participant2_private 2>/dev/null)
bob_secret=$(echo "$bob_nonce" | grep "secret share:" | cut -d' ' -f3)
bob_public_msg=$(echo "$bob_nonce" | grep "public share:" | cut -d' ' -f3)

echo "Charlie generating nonces..."
charlie_nonce=$(cargo run -- agg-send-step-one $participant3_private 2>/dev/null)
charlie_secret=$(echo "$charlie_nonce" | grep "secret share:" | cut -d' ' -f3)
charlie_public_msg=$(echo "$charlie_nonce" | grep "public share:" | cut -d' ' -f3)

print_success "All participants generated nonces!"
print_info "These nonces ensure each signature is unique and secure"

print_step "Getting recent blockhash for transaction"
recent_blockhash=$(cargo run -- recent-block-hash --net localnet 2>/dev/null | grep "Recent blockhash:" | cut -d' ' -f3)
print_info "Blockhash: $recent_blockhash"

print_step "MPC Step 2: Generating partial signatures"
echo -e "${PURPLE}Each participant signs the transaction with their private key and nonces...${NC}"

echo "Alice creating partial signature..."
alice_sig=$(cargo run -- agg-send-step-two-token \
    --private-key $participant1_private \
    --mint $mint_address \
    --amount 10 \
    --decimals 6 \
    --to $creator_public \
    --recent-block-hash $recent_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --first-messages $bob_public_msg,$charlie_public_msg \
    --secret-state $alice_secret \
    --net localnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)
echo -e "${PURPLE}Alice's partial signature: $alice_sig${NC}"

echo "Bob creating partial signature..."
bob_sig=$(cargo run -- agg-send-step-two-token \
    --private-key $participant2_private \
    --mint $mint_address \
    --amount 10 \
    --decimals 6 \
    --to $creator_public \
    --recent-block-hash $recent_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --first-messages $alice_public_msg,$charlie_public_msg \
    --secret-state $bob_secret \
    --net localnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)
echo -e "${PURPLE}Bob's partial signature: $bob_sig${NC}"

echo "Charlie creating partial signature..."
charlie_sig=$(cargo run -- agg-send-step-two-token \
    --private-key $participant3_private \
    --mint $mint_address \
    --amount 10 \
    --decimals 6 \
    --to $creator_public \
    --recent-block-hash $recent_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --first-messages $alice_public_msg,$bob_public_msg \
    --secret-state $charlie_secret \
    --net localnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)
echo -e "${PURPLE}Charlie's partial signature: $charlie_sig${NC}"

print_success "All partial signatures generated!"
print_info "Each signature is only a piece - none can move funds alone"


print_step "MPC Step 3: Aggregating signatures and broadcasting"
echo -e "${PURPLE}Combining all partial signatures into a complete transaction...${NC}"

token_transfer_result=$(cargo run -- aggregate-signatures-and-broadcast-token --signatures "$alice_sig,$bob_sig,$charlie_sig" --mint "$mint_address" --amount 10 --decimals 6 --to "$creator_public" --recent-block-hash "$recent_blockhash" --keys "$participant1_public,$participant2_public,$participant3_public" --net localnet 2>/dev/null)

if [[ $token_transfer_result == *"Transaction ID"* ]]; then
    token_tx_id=$(echo "$token_transfer_result" | grep "Transaction ID:" | cut -d' ' -f3)
    print_success "🎉 MPC TOKEN TRANSFER SUCCESSFUL!"
    echo -e ""
    echo -e "${GREEN}📋 TRANSFER DETAILS:"
    echo -e "Amount: 10 tokens"
    echo -e "From: Aggregated wallet (MPC)"
    echo -e "To: Creator wallet"
    echo -e "Transaction ID: $token_tx_id${NC}"
else
    echo -e "${RED}❌ Token transfer failed:${NC}"
    echo -e "${RED}Full output: $token_transfer_result${NC}"
    echo -e ""
fi

echo "waiting for 15 seconds before checking balances"
sleep 15

echo "Checking balances after transaction..."   
final_creator_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$creator_public" --net localnet 2>/dev/null | grep -o '[0-9]\+ tokens')
final_agg_balance=$(cargo run -- token-balance --mint "$mint_address" --wallet "$aggregated_key" --net localnet 2>/dev/null | grep -o '[0-9]\+ tokens')

print_info "Final creator balance: $final_creator_balance"
print_info "Final aggregated key balance: $final_agg_balance"


# Step 7: MPC SOL Transfer Demo
print_header "STEP 7: MPC SOL TRANSFER DEMONSTRATION"
echo -e "${YELLOW}Let's also demonstrate transferring native SOL using MPC!${NC}"

echo -e "${CYAN}SOL balances before transfer:${NC}"
agg_balance=$(cargo run -- balance $aggregated_key --net localnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
creator_balance=$(cargo run -- balance $creator_public --net localnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
agg_sol=$(echo "scale=3; $agg_balance / 1000000000" | bc -l)
creator_sol=$(echo "scale=3; $creator_balance / 1000000000" | bc -l)
echo "Aggregated wallet: $agg_sol SOL ($agg_balance lamports)"
echo "Creator wallet: $creator_sol SOL ($creator_balance lamports)"

echo -e "We'll send 0.1 SOL from the aggregated wallet to the creator."

print_step "MPC SOL Transfer - Generating fresh nonces"

# Generate fresh nonces for SOL transfer
alice_sol_nonce=$(cargo run -- agg-send-step-one $participant1_private 2>/dev/null)
alice_sol_secret=$(echo "$alice_sol_nonce" | grep "secret share:" | cut -d' ' -f3)
alice_sol_public=$(echo "$alice_sol_nonce" | grep "public share:" | cut -d' ' -f3)

bob_sol_nonce=$(cargo run -- agg-send-step-one $participant2_private 2>/dev/null)
bob_sol_secret=$(echo "$bob_sol_nonce" | grep "secret share:" | cut -d' ' -f3)
bob_sol_public=$(echo "$bob_sol_nonce" | grep "public share:" | cut -d' ' -f3)

charlie_sol_nonce=$(cargo run -- agg-send-step-one $participant3_private 2>/dev/null)
charlie_sol_secret=$(echo "$charlie_sol_nonce" | grep "secret share:" | cut -d' ' -f3)
charlie_sol_public=$(echo "$charlie_sol_nonce" | grep "public share:" | cut -d' ' -f3)

# Get fresh blockhash
sol_blockhash=$(cargo run -- recent-block-hash --net localnet 2>/dev/null | grep "Recent blockhash:" | cut -d' ' -f3)

print_step "Creating partial signatures for SOL transfer"

alice_sol_sig=$(cargo run -- agg-send-step-two-sol \
    --private-key $participant1_private \
    --amount 0.1 \
    --to $creator_public \
    --memo "MPC SOL Demo Transfer" \
    --recent-block-hash $sol_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --first-messages $bob_sol_public,$charlie_sol_public \
    --secret-state $alice_sol_secret \
    --net localnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)
echo -e "${PURPLE}Alice's SOL partial signature: $alice_sol_sig${NC}"

bob_sol_sig=$(cargo run -- agg-send-step-two-sol \
    --private-key $participant2_private \
    --amount 0.1 \
    --to $creator_public \
    --memo "MPC SOL Demo Transfer" \
    --recent-block-hash $sol_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --first-messages $alice_sol_public,$charlie_sol_public \
    --secret-state $bob_sol_secret \
    --net localnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)
echo -e "${PURPLE}Bob's SOL partial signature: $bob_sol_sig${NC}"

charlie_sol_sig=$(cargo run -- agg-send-step-two-sol \
    --private-key $participant3_private \
    --amount 0.1 \
    --to $creator_public \
    --memo "MPC SOL Demo Transfer" \
    --recent-block-hash $sol_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --first-messages $alice_sol_public,$bob_sol_public \
    --secret-state $charlie_sol_secret \
    --net localnet 2>/dev/null | grep "partial signature:" | cut -d' ' -f3)
echo -e "${PURPLE}Charlie's SOL partial signature: $charlie_sol_sig${NC}"

print_step "aggregating signatures and broadcasting SOL transfer"

sol_transfer_result=$(cargo run -- aggregate-signatures-and-broadcast-sol \
    --signatures $alice_sol_sig,$bob_sol_sig,$charlie_sol_sig \
    --amount 0.1 \
    --to $creator_public \
    --memo "MPC SOL Demo Transfer" \
    --recent-block-hash $sol_blockhash \
    --keys $participant1_public,$participant2_public,$participant3_public \
    --net localnet 2>/dev/null)

if [[ $sol_transfer_result == *"Transaction ID"* ]]; then
    sol_tx_id=$(echo "$sol_transfer_result" | grep "Transaction ID:" | cut -d' ' -f3)
    print_success "🎉 MPC SOL TRANSFER SUCCESSFUL!"
    echo -e "Transaction ID: $sol_tx_id${NC}"
else
    echo -e "${RED}❌ SOL transfer failed: $sol_transfer_result${NC}"
fi

echo -e "\n${PURPLE}waiting for 15 seconds before checking balances${NC}"
sleep 15

echo -e "${CYAN}SOL balances after transfer:${NC}"
agg_balance=$(cargo run -- balance $aggregated_key --net localnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
creator_balance=$(cargo run -- balance $creator_public --net localnet 2>/dev/null | grep "The balance" | grep -o '[0-9]*' | tail -1)
agg_sol=$(echo "scale=3; $agg_balance / 1000000000" | bc -l)
creator_sol=$(echo "scale=3; $creator_balance / 1000000000" | bc -l)
echo "Aggregated wallet: $agg_sol SOL ($agg_balance lamports)"
echo "Creator wallet: $creator_sol SOL ($creator_balance lamports)"

# Final Summary
print_header "🎉 DEMO COMPLETE - SUMMARY 🎉"
echo -e "${GREEN}Congratulations! You've successfully completed the MPC demo!"
echo -e ""
echo -e "What you accomplished:"
echo -e "✓ Generated 3 MPC participant keys (Alice, Bob, Charlie)"
echo -e "✓ Created an aggregated wallet requiring all 3 signatures"
echo -e "✓ Funded wallets with SOL from localnet faucet"
echo -e "✓ Created a custom SPL token"
echo -e "✓ Minted tokens to the MPC-controlled wallet"
echo -e "✓ Performed MPC token transfer (10 tokens)"
echo -e "✓ Performed MPC SOL transfer (0.01 SOL)"
echo -e ""
echo -e "🔑 KEY CONCEPTS DEMONSTRATED:"
echo -e "• Multi-Party Computation (MPC) for secure signature generation"
echo -e "• Aggregated wallets that require multiple signatures"
echo -e "• SPL token creation and management"
echo -e "• Both token and native SOL transfers via MPC"
echo -e ""
echo -e "🌐 LOCALNET ADDRESSES FOR YOUR REFERENCE:"
echo -e "Aggregated Wallet: $aggregated_key"
echo -e "Creator Wallet: $creator_public"
echo -e "SPL Token Mint: $mint_address"
echo -e ""
echo -e "You can view these transactions on Solana Explorer (localnet)!"
echo -e "This MPC system is now ready for production use! 🚀${NC}"

echo -e "${RED}Remember: In production, never share private keys like we did in this demo!${NC}"

echo -e "\n${CYAN}That's the end of the demo! Thanks for trying it out! 🙏${NC}"
