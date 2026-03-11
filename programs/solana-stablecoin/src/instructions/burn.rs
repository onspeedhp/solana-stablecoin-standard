use anchor_lang::prelude::*;
use anchor_spl::token_2022::{burn, Burn, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
pub struct BurnStablecoin<'info> {
    pub burner: Signer<'info>,

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
    pub from: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"role", config.key().as_ref(), burner.key().as_ref()],
        bump = role_account.bump
    )]
    pub role_account: Option<Account<'info, RoleAccount>>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<BurnStablecoin>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    let burner = ctx.accounts.burner.key();

    // Access Control: Must be admin or have BURNER role
    let has_role = if let Some(role_account) = &ctx.accounts.role_account {
        (role_account.roles & roles::BURNER) != 0
    } else {
        false
    };

    if burner != config.admin && !has_role {
        return err!(StablecoinError::Unauthorized);
    }

    // CPI to Token-2022
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.from.to_account_info(),
        authority: ctx.accounts.burner.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    burn(cpi_ctx, amount)?;

    msg!("Burned {} tokens from {}", amount, ctx.accounts.from.key());
    Ok(())
}
