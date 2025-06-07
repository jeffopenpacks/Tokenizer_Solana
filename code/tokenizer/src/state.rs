use anchor_lang::prelude::*;

#[account]
pub struct CustomMint {
    pub name: String,
    pub symbol: String,
    pub max_supply: Option<u64>,
    pub owners: Option<Vec<Pubkey>>,
    pub threshold: Option<u8>,
}

#[account]
pub struct CustomPDA {
    pub authority: Pubkey,
}

impl CustomMint {
    pub const LEN: usize = 
        4 + 32 + // name (max 32 chars)
        4 + 10 + //symbol(max 10 chars)
        1 + 8 + //Option<u64> (1 for enum + 8 bytes)
        1 + 4 + (32 * 5) + // owners (max 5 owners)
        1 + 1; // threshold 
}


#[account]
pub struct UserTokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

impl UserTokenAccount {
    pub const LEN: usize =
        8 +
        32 +
        32 +
        8;
}

#[event]
pub struct MintInitialised {
    pub custom_mint: Pubkey,
    pub spl_mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub max_supply: u64,
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
}