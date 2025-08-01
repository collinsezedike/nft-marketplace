use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, MintTo, TokenInterface, TokenAccount, mint_to },
};

use crate::state::{ Marketplace, UserAccount };

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards,
        associated_token::authority = user
    )]
    pub user_reward_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(mut, seeds = [b"reward", marketplace.key().as_ref()], bump = marketplace.rewards_bump)]
    pub rewards: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.rewards.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };

        let seeds = [
            b"marketplace",
            self.marketplace.name.as_str().as_bytes(),
            &[self.marketplace.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, self.user_account.points as u64)?;

        // Reset the user's points
        self.user_account.points = 0;

        Ok(())
    }
}
