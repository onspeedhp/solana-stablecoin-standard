use anchor_lang::prelude::*;
use anchor_spl::token_2022::{mint_to, MintTo, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    pub minter: Signer<'info>,

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
    pub to: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"role", role_types::MINTER.as_bytes(), minter.key().as_ref()],
        bump = role_account.bump
    )]
    pub role_account: Account<'info, RoleAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<MintStablecoin>, amount: u64) -> Result<()> {
    if ctx.accounts.role_account.role_type != role_types::MINTER {
        return err!(StablecoinError::Unauthorized);
    }

    // CPI to Token-2022
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.minter.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    mint_to(cpi_ctx, amount)?;

    msg!("Minted {} tokens to {}", amount, ctx.accounts.to.key());
    Ok(())
}
