use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Execute<'info> {
    #[account(
        token::mint = mint,
        token::program = token_program,
    )]
    pub source: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        address = config.mint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = mint,
        token::program = token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    
    /// CHECK: Transfer authority
    pub owner: UncheckedAccount<'info>,

    /// CHECK: Validation account (not used here but required by interface)
    pub extra_metas: UncheckedAccount<'info>,

    #[account(
        seeds = [b"config", mint.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        seeds = [b"pause", config.key().as_ref()],
        bump = pause_state.bump
    )]
    pub pause_state: Option<Account<'info, PauseState>>,

    #[account(
        seeds = [b"blacklist", config.key().as_ref(), source.owner.as_ref()],
        bump = source_blacklist.bump
    )]
    pub source_blacklist: Option<Account<'info, BlacklistAccount>>,

    #[account(
        seeds = [b"blacklist", config.key().as_ref(), destination.owner.as_ref()],
        bump = destination_blacklist.bump
    )]
    pub destination_blacklist: Option<Account<'info, BlacklistAccount>>,

    pub token_program: Interface<'info, anchor_spl::token_interface::TokenInterface>,
}

pub fn handler(ctx: Context<Execute>, _amount: u64) -> Result<()> {
    // 1. Check Global Pause
    if let Some(pause_state) = &ctx.accounts.pause_state {
        if pause_state.is_paused {
            return err!(StablecoinError::Paused);
        }
    }

    // 2. Check Blacklist for source
    if let Some(blacklist) = &ctx.accounts.source_blacklist {
        if blacklist.is_blacklisted {
            return err!(StablecoinError::Blacklisted);
        }
    }

    // 3. Check Blacklist for destination
    if let Some(blacklist) = &ctx.accounts.destination_blacklist {
        if blacklist.is_blacklisted {
            return err!(StablecoinError::Blacklisted);
        }
    }

    Ok(())
}
