use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenInterface };

use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
        space = 8 + Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(seeds = [b"treasury", marketplace.key().as_ref()], bump)]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"reward", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace
    )]
    pub rewards: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            bump: bumps.marketplace,
            rewards_bump: bumps.rewards,
            treasury_bump: bumps.treasury,
            fee,
            admin: self.admin.key(),
            name,
        });

        Ok(())
    }
}
