use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    // token::{ close_account, transfer_checked, CloseAccount, TransferChecked },
    token_interface::{
        close_account,
        transfer_checked,
        CloseAccount,
        Mint,
        TokenAccount,
        TokenInterface,
        TransferChecked,
    },
};

use crate::state::{ Listing, Marketplace };

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub lister: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub lister_nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = lister_nft_mint,
        associated_token::authority = lister,
        associated_token::program = token_program,
    )]
    pub lister_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = lister,
        seeds = [b"listing", marketplace.key().as_ref(), lister_nft_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = lister_nft_mint,
        associated_token::authority = listing,
        associated_token::program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn delist(&mut self) -> Result<()> {
        self.withdraw_nft()?;
        self.close_vault()?;
        Ok(())
    }

    pub fn withdraw_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.lister_nft_mint.to_account_info(),
            to: self.lister_nft_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            b"listing",
            self.marketplace.to_account_info().key.as_ref(),
            self.lister_nft_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.lister_nft_mint.decimals)?;

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        // let cpi_program = self.associated_token_program.to_account_info();
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.lister.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            b"listing",
            self.marketplace.to_account_info().key.as_ref(),
            self.lister_nft_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
