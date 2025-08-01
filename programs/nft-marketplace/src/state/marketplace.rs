use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub bump: u8,
    pub rewards_bump: u8,
    pub treasury_bump: u8,
    pub points_per_purchase: u16,
    pub fee: u16,
    pub admin: Pubkey,
    #[max_len(32)]
    pub name: String,
}
