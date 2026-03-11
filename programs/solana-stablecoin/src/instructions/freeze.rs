use anchor_lang::prelude::*;
use anchor_spl::token_2022::{freeze_account, thaw_account, FreezeAccount, ThawAccount, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
pub struct ManageFreeze<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config", mint.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        address = config.mint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::program = token_program,
    )]
    pub account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn freeze_handler(ctx: Context<ManageFreeze>) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();

    if authority != config.admin && authority != config.freeze_authority {
        return err!(StablecoinError::Unauthorized);
    }

    let cpi_accounts = FreezeAccount {
        account: ctx.accounts.account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    freeze_account(cpi_ctx)?;

    msg!("Account frozen: {}", ctx.accounts.account.key());
    Ok(())
}

pub fn thaw_handler(ctx: Context<ManageFreeze>) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();

    if authority != config.admin && authority != config.freeze_authority {
        return err!(StablecoinError::Unauthorized);
    }

    let cpi_accounts = ThawAccount {
        account: ctx.accounts.account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    thaw_account(cpi_ctx)?;

    msg!("Account thawed: {}", ctx.accounts.account.key());
    Ok(())
}
