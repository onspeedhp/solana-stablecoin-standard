use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct UpdateRoles<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config", config.mint.as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + RoleAccount::INIT_SPACE,
        seeds = [b"role", config.key().as_ref(), user.as_ref()],
        bump
    )]
    pub role_account: Account<'info, RoleAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateRoles>, user: Pubkey, roles: u16) -> Result<()> {
    let config = &ctx.accounts.config;
    let admin = ctx.accounts.admin.key();

    if admin != config.admin {
        return err!(StablecoinError::Unauthorized);
    }

    let role_account = &mut ctx.accounts.role_account;
    role_account.user = user;
    role_account.roles = roles;
    role_account.bump = ctx.bumps.role_account;

    msg!("Roles updated for user {}: {}", user, roles);
    Ok(())
}
