use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub bump: u8,
    pub price: u32,
    pub mint: Pubkey,
    pub maker: Pubkey,
}
