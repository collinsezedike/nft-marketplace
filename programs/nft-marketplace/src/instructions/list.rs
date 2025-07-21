use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{ MasterEditionAccount, Metadata, MetadataAccount },
    // token::{ transfer_checked, TransferChecked },
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::state::{ Listing, Marketplace };

#[derive(Accounts)]
pub struct List<'info> {
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
        init,
        payer = lister,
        seeds = [b"listing", marketplace.key().as_ref(), lister_nft_mint.key().as_ref()],
        bump,
        space = 8 + Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        init,
        payer = lister,
        associated_token::mint = lister_nft_mint,
        associated_token::authority = listing,
        associated_token::program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account{
        seeds = [b"metadata", metadata_program.key().as_ref(), lister_nft_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    }]
    pub metadata: Account<'info, MetadataAccount>,

    #[account{
        seeds = [b"metadata", metadata_program.key().as_ref(), lister_nft_mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump
    }]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn list(&mut self, price: u32, bumps: &ListBumps) -> Result<()> {
        self.initialize_listing(price, bumps.listing)?;
        self.deposit_nft()?;
        Ok(())
    }

    pub fn initialize_listing(&mut self, price: u32, bump: u8) -> Result<()> {
        self.listing.set_inner(Listing {
            bump,
            price,
            mint: self.lister_nft_mint.key(),
            maker: self.lister.key(),
        });

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.lister_nft_ata.to_account_info(),
            mint: self.lister_nft_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.lister.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.lister_nft_ata.amount, self.lister_nft_mint.decimals)?;

        Ok(())
    }
}
