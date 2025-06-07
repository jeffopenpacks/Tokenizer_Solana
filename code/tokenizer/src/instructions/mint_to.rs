use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::*;
use crate::CustomError;
use anchor_lang::solana_program::program_pack::IsInitialized;

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"custom_mint", spl_mint.key().as_ref()],
        bump
    )]
    pub custom_mint: Account<'info, CustomMint>,

    #[account(mut)]
    pub spl_mint: Account<'info, Mint>,

    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"mint_auth", spl_mint.key().as_ref()],
        bump
    )]
    /// CHECK: PDA mint authority
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_to_handler(
    ctx: Context<MintTo>,
    amount: u64,
    signers: Vec<Pubkey>,
) -> Result<()> {
    // Check that payer is one of the owners
    require!(
        ctx.accounts.custom_mint.owners.as_ref().unwrap().contains(&ctx.accounts.payer.key()),
        CustomError::NotOwner
    );

    // Check that recipient token account is initialized
    require!(
        ctx.accounts.recipient.is_initialized(),
        CustomError::UninitializedTokenAccount
    );

    require!(
        ctx.accounts.spl_mint.supply + amount <= ctx.accounts.custom_mint.max_supply.unwrap(),
        CustomError::ExceedsMaxSupply
    );

    // Check multisig threshold
    let mut approved = 0;
    for owner in ctx.accounts.custom_mint.owners.as_ref().unwrap() {
        if signers.contains(owner) {
            approved += 1;
            msg!("Owner {} approved", owner);
            msg!("Total approved: {}", approved);
        }
    }
    require!(
        approved >= ctx.accounts.custom_mint.threshold.unwrap(),
        CustomError::ThresholdNotMet
    );

    // Derive PDA signer for mint authority
    let binding = ctx.accounts.spl_mint.key();
    let bump = *ctx.bumps.get("mint_authority").unwrap();
    let signer_seeds: &[&[u8]] = &[
        b"mint_auth",
        binding.as_ref(),
        &[bump],
    ];

    // CPI to SPL Token Program's mint_to
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.spl_mint.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let binding = [signer_seeds];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        &binding,
    );
    token::mint_to(cpi_ctx, amount)?;

    Ok(())
}
