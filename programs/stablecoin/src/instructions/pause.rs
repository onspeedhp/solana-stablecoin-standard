use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
pub struct ManagePause<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config", config.mint.as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + PauseState::INIT_SPACE,
        seeds = [b"pause", config.key().as_ref()],
        bump
    )]
    pub pause_state: Account<'info, PauseState>,

    #[account(
        seeds = [b"role", role_types::PAUSER.as_bytes(), authority.key().as_ref()],
        bump = role_account.bump
    )]
    pub role_account: Account<'info, RoleAccount>,

    pub system_program: Program<'info, System>,
}

pub fn pause_handler(ctx: Context<ManagePause>) -> Result<()> {
    if ctx.accounts.role_account.role_type != role_types::PAUSER {
        return err!(StablecoinError::Unauthorized);
    }

    ctx.accounts.pause_state.is_paused = true;
    ctx.accounts.pause_state.bump = ctx.bumps.pause_state;

    msg!("Stablecoin paused");
    Ok(())
}

pub fn unpause_handler(ctx: Context<ManagePause>) -> Result<()> {
    if ctx.accounts.role_account.role_type != role_types::PAUSER {
        return err!(StablecoinError::Unauthorized);
    }

    ctx.accounts.pause_state.is_paused = false;
    ctx.accounts.pause_state.bump = ctx.bumps.pause_state;

    msg!("Stablecoin unpaused");
    Ok(())
}
