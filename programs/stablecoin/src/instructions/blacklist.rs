use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct ManageBlacklist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config", config.mint.as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + BlacklistAccount::INIT_SPACE,
        seeds = [b"blacklist", config.key().as_ref(), user.as_ref()],
        bump
    )]
    pub blacklist_account: Account<'info, BlacklistAccount>,

    #[account(
        seeds = [b"role", role_types::BLACKLISTER.as_bytes(), authority.key().as_ref()],
        bump = role_account.bump
    )]
    pub role_account: Account<'info, RoleAccount>,

    pub system_program: Program<'info, System>,
}

pub fn add_handler(ctx: Context<ManageBlacklist>, user: Pubkey, reason: String) -> Result<()> {
    if ctx.accounts.role_account.role_type != role_types::BLACKLISTER {
        return err!(StablecoinError::Unauthorized);
    }

    let blacklist = &mut ctx.accounts.blacklist_account;
    blacklist.wallet = user;
    blacklist.reason = reason;
    blacklist.created_at = Clock::get()?.unix_timestamp;
    blacklist.is_blacklisted = true;
    blacklist.bump = ctx.bumps.blacklist_account;

    msg!("User {} added to blacklist", user);
    Ok(())
}

pub fn remove_handler(ctx: Context<ManageBlacklist>, _user: Pubkey) -> Result<()> {
    if ctx.accounts.role_account.role_type != role_types::BLACKLISTER {
        return err!(StablecoinError::Unauthorized);
    }

    let blacklist = &mut ctx.accounts.blacklist_account;
    blacklist.is_blacklisted = false;

    msg!("User removed from blacklist");
    Ok(())
}
