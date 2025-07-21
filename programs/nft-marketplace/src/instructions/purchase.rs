use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };
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
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub lister: SystemAccount<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut, 
        seeds = [b"treasury", marketplace.key().as_ref()], 
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        // close = listing.maker.key(),
        close = lister,
        seeds = [b"listing", marketplace.key().as_ref(), nft_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn purchase(&mut self) -> Result<()> {
        let marketplace_fees = ((self.marketplace.fee / 10_000) as u32) * self.listing.price;
        let amount_to_pay_lister = self.listing.price - marketplace_fees;

        self.transfer_nft()?;
        self.take_marketplace_fees(marketplace_fees as u64)?;
        self.pay_lister(amount_to_pay_lister as u64)?;
        self.close_vault()?;
        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            to: self.buyer_nft_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            b"listing",
            self.marketplace.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.nft_mint.decimals)?;

        Ok(())
    }

    pub fn take_marketplace_fees(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn pay_lister(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.lister.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

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
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
