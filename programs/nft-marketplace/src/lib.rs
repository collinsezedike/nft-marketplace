pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2H8vbQag5vs4RXmmzDUGQ6Uu3KMzbLKC6LQG99t17TrV");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        fee: u16,
        points_per_purchase: u16
    ) -> Result<()> {
        ctx.accounts.initialize(name, fee, points_per_purchase, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u32) -> Result<()> {
        ctx.accounts.list(price, &ctx.bumps)
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase()
    }

    pub fn claim_rewards(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim_rewards()
    }
}
