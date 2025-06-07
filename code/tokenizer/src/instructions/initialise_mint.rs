use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::CreateV1CpiBuilder,
    types::{PrintSupply, TokenStandard},
};
use crate::state::*;
use crate::CustomError;
use solana_program::sysvar::instructions;


#[derive(Accounts)]
#[instruction(name: String, symbol: String, max_supply: Option<u64>, owners: Option<Vec<Pubkey>>, threshold: Option<u8>)]

pub struct InitialiseMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + CustomMint::LEN,
        seeds = [b"custom_mint", spl_mint.key().as_ref()],
        bump,
    )]
    pub custom_mint: Account<'info, CustomMint>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub spl_mint: Account<'info, Mint>,

    #[account(seeds = [b"mint_auth", spl_mint.key().as_ref()], bump)]
    /// CHECK: This is a PDA
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Metaplex Metadata will be derived inside
    pub metadata: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(address = mpl_token_metadata::ID)]
    /// CHECK: Metaplex token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,

    #[account(address = instructions::ID)]
    /// CHECK: Instructions sysvar
    pub sysvar_instructions: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn init_mint_handler(
    ctx: Context<InitialiseMint>, 
    name: String, 
    symbol: String, 
    max_supply: Option<u64>,
    owners: Option<Vec<Pubkey>>,
    threshold: Option<u8>,
) -> Result<()> {

    require!(name.len() <= 32, CustomError::NameTooLong);
    require!(symbol.len() <= 10, CustomError::SymbolTooLong);

    msg!("Name: {}, Symbol: {}", name, symbol);
    let name_for_event = name.clone();
    let symbol_for_event = symbol.clone();
    let custom_mint = &mut ctx.accounts.custom_mint;

    msg!("Assigned custom mint");
    custom_mint.name = name.clone();
    custom_mint.symbol = symbol.clone();

    let max_supply_value = max_supply.unwrap_or(1_000_000_000);
    let max_supply_for_event = max_supply_value.clone();
    require!(max_supply_value > 0 && max_supply_value < u64::MAX, CustomError::InvalidMaxSupply);
    custom_mint.max_supply = Some(max_supply_value);

    if let Some(mut o) = owners {
        if !o.contains(&ctx.accounts.payer.key()) {
            o.push(ctx.accounts.payer.key());
        }
        require!(o.len() <= 5, CustomError::MaxOwnersExceeded);
        let threshold_value = threshold.unwrap_or(1);

        require!(threshold_value <= o.len() as u8, CustomError::ThresholdExceedsOwners);
        custom_mint.owners = Some(o);

        custom_mint.threshold = Some(threshold_value);
    } else {
        custom_mint.owners = Some(vec![ctx.accounts.payer.key()]);
        custom_mint.threshold = Some(1);
    }
    let threshold_for_event = custom_mint.threshold.clone().unwrap();
    let owners_for_event = custom_mint.owners.clone().unwrap();

    msg!("Generating metadata PDA");
        let binding = ctx.accounts.spl_mint.key();
        let signer_seeds = &[
            b"mint_auth",
            binding.as_ref(),
            &[*ctx.bumps.get("mint_authority").unwrap()],
        ];
    CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .metadata(&ctx.accounts.metadata)
        .mint(&ctx.accounts.spl_mint.to_account_info(), true)
        .authority(&ctx.accounts.mint_authority)
        .payer(&ctx.accounts.payer)
        .update_authority(&ctx.accounts.mint_authority, false)
        .system_program(&ctx.accounts.system_program)
        .spl_token_program(Some(&ctx.accounts.token_program))
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .name(name.clone())
        .symbol(symbol.clone())
        .uri("".to_string())
        .seller_fee_basis_points(0)
        .token_standard(TokenStandard::Fungible)
        .print_supply(PrintSupply::Zero)
        .invoke_signed(&[signer_seeds])?;

    emit!(MintInitialised {
        custom_mint: ctx.accounts.custom_mint.key(),
        spl_mint: ctx.accounts.spl_mint.key(),
        name: name_for_event,
        symbol: symbol_for_event,
        max_supply: max_supply_for_event,
        owners: owners_for_event,
        threshold: threshold_for_event,
    });
    msg!("Mint + Metadata successfully initialized.");
    Ok(())
}