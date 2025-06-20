use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction,
    transaction::Transaction,
    program_pack::Pack,
};
use spl_token::{
    instruction::{initialize_mint, mint_to_checked, transfer},
    state::Mint,
};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};

use crate::error::Error;

/// Create a new SPL token mint
pub fn create_token_mint(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
) -> Result<(Pubkey, Signature), Error> {
    // Generate a new keypair for the mint account
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();
    
    // Calculate the minimum rent for a mint account
    let mint_rent = rpc_client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .map_err(Error::RecentHashFailed)?;
    
    // Create the mint account
    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &mint_pubkey,
        mint_rent,
        Mint::LEN as u64,
        &spl_token::id(),
    );
    
    // Initialize the mint
    let initialize_mint_instruction = initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        mint_authority,
        freeze_authority,
        decimals,
    )
    .map_err(|e| Error::TokenCreationFailed(format!("Failed to create initialize mint instruction: {}", e)))?;
    
    // Create and send transaction
    let instructions = vec![create_account_instruction, initialize_mint_instruction];
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(Error::RecentHashFailed)?;
        
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash,
    );
    
    let signature = rpc_client
        .send_transaction(&transaction)
        .map_err(|e| Error::TokenCreationFailed(e.to_string()))?;
        
    Ok((mint_pubkey, signature))
}

/// Mint tokens to a destination account
pub fn mint_tokens_to(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    destination: &Pubkey,
    mint_authority: &Keypair,
    amount: u64,
    decimals: u8,
) -> Result<Signature, Error> {
    // Get or create the destination's associated token account
    let destination_ata = get_associated_token_address(destination, mint);
    
    let mut instructions = Vec::new();
    
    // Check if the destination ATA exists, if not create it
    if rpc_client.get_account(&destination_ata).is_err() {
        let create_ata_instruction = create_associated_token_account(
            &payer.pubkey(),
            destination,
            mint,
            &spl_token::id(),
        );
        instructions.push(create_ata_instruction);
    }
    
    // Create mint instruction
    let mint_instruction = mint_to_checked(
        &spl_token::id(),
        mint,
        &destination_ata,
        &mint_authority.pubkey(),
        &[&mint_authority.pubkey()],
        amount,
        decimals,
    )
    .map_err(|e| Error::TokenMintFailed(format!("Failed to create mint instruction: {}", e)))?;
    
    instructions.push(mint_instruction);
    
    // Create and send transaction
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(Error::RecentHashFailed)?;
        
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer, mint_authority],
        recent_blockhash,
    );
    
    let signature = rpc_client
        .send_transaction(&transaction)
        .map_err(|e| Error::TokenMintFailed(e.to_string()))?;
        
    Ok(signature)
}

/// Transfer tokens from one wallet to another
pub fn transfer_tokens(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    from_wallet: &Keypair,
    to_wallet: &Pubkey,
    amount: u64,
) -> Result<Signature, Error> {
    // Get associated token addresses
    let from_ata = get_associated_token_address(&from_wallet.pubkey(), mint);
    let to_ata = get_associated_token_address(to_wallet, mint);
    
    let mut instructions = Vec::new();
    
    // Check if sender ATA exists
    if rpc_client.get_account(&from_ata).is_err() {
        return Err(Error::TokenAccountNotFound);
    }
    
    // Create destination ATA if it doesn't exist
    if rpc_client.get_account(&to_ata).is_err() {
        let create_ata_instruction = create_associated_token_account(
            &payer.pubkey(),
            to_wallet,
            mint,
            &spl_token::id(),
        );
        instructions.push(create_ata_instruction);
    }
    
    // Create transfer instruction
    let transfer_instruction = transfer(
        &spl_token::id(),
        &from_ata,
        &to_ata,
        &from_wallet.pubkey(),
        &[&from_wallet.pubkey()],
        amount,
    )
    .map_err(|e| Error::TokenTransferFailed(format!("Failed to create transfer instruction: {}", e)))?;
    
    instructions.push(transfer_instruction);
    
    // Create and send transaction
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(Error::RecentHashFailed)?;
        
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer, from_wallet],
        recent_blockhash,
    );
    
    let signature = rpc_client
        .send_transaction(&transaction)
        .map_err(|e| Error::TokenTransferFailed(e.to_string()))?;
        
    Ok(signature)
}

/// Get the token balance of a wallet for a specific mint
pub fn get_token_balance(
    rpc_client: &RpcClient,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> Result<u64, Error> {
    let ata = get_associated_token_address(wallet, mint);
    
    match rpc_client.get_token_account_balance(&ata) {
        Ok(balance) => Ok(balance.amount.parse().unwrap_or(0)),
        Err(_) => Ok(0), // Account doesn't exist, so balance is 0
    }
}

/// Helper function to get associated token address
pub fn get_ata_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    get_associated_token_address(wallet, mint)
} 