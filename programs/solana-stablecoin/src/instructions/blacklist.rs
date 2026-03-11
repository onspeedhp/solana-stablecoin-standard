use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct ManageBlacklist<'info> {
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

    pub system_program: Program<'info, System>,
}

pub fn add_handler(ctx: Context<ManageBlacklist>, user: Pubkey) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();

    // Only Admin or Blacklister role can manage blacklist
    // (Assuming a simple check here, could be expanded to use RoleAccount)
    if authority != config.admin && authority != config.blacklist_authority {
        return err!(StablecoinError::Unauthorized);
    }

    let blacklist = &mut ctx.accounts.blacklist_account;
    blacklist.user = user;
    blacklist.is_blacklisted = true;
    blacklist.bump = ctx.bumps.blacklist_account;

    msg!("User {} added to blacklist", user);
    Ok(())
}

pub fn remove_handler(ctx: Context<ManageBlacklist>, user: Pubkey) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();

    if authority != config.admin && authority != config.blacklist_authority {
        return err!(StablecoinError::Unauthorized);
    }

    let blacklist = &mut ctx.accounts.blacklist_account;
    blacklist.is_blacklisted = false;

    msg!("User {} removed from blacklist", user);
    Ok(())
}
