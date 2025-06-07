use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;


declare_id!("ErCER1skeaeqzRzSaqpmAKEAZmGGdh9VRzNAXXWUXv8f");

use instructions::initialise_mint::*;
use instructions::mint_to::*;

#[error_code]
pub enum CustomError {
    #[msg("Threshold exceeds the number of owners.")]
    ThresholdExceedsOwners,
    #[msg("Max supply is invalid.")]
    InvalidMaxSupply,
    #[msg("Not enough signers for the transaction.")]
    NotEnoughSigners,
    #[msg("Signer not found in the provided signers list.")]
    SignerNotFound,
    #[msg("Minting exceeds the maximum supply.")]
    ExceedsMaxSupply,
    #[msg("Threshold not met for the transaction.")]
    ThresholdNotMet,
    #[msg("Uninitialized token account.")]
    UninitializedTokenAccount,
    #[msg("Name is too long.")]
    NameTooLong,
    #[msg("Symbol is too long.")]
    SymbolTooLong,
    #[msg("Too many owners provided.")]
    MaxOwnersExceeded,
    #[msg("WHO ARE YOU? You are not the owner of this token.")]
    NotOwner,
}

#[program]
pub mod tokenizer {
    use super::*;

    pub fn initialise_mint(ctx: Context<InitialiseMint>, name: String, symbol: String, max_supply: Option<u64>, owners: Option<Vec<Pubkey>>, threshold: Option<u8>) -> Result<()> {
        init_mint_handler(ctx, name, symbol, max_supply, owners, threshold)
    }

   pub fn mint_to(ctx: Context<MintTo>, amount: u64, signers: Vec<Pubkey>) -> Result<()> {
        mint_to_handler(ctx, amount, signers)
    }
}