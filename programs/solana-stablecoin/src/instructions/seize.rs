use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
pub struct SeizeStablecoin<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config", mint.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        mut,
        address = config.mint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::program = token_program,
    )]
    pub source: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::program = token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<SeizeStablecoin>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();

    // Only Admin or compliance-authorized role can seize
    if authority != config.admin {
        return err!(StablecoinError::Unauthorized);
    }

    // CPI to Token-2022: Transfer from source to destination using the permanent delegate authority
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.source.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

    msg!("Seized {} tokens from {} to {}", amount, ctx.accounts.source.key(), ctx.accounts.destination.key());
    Ok(())
}
