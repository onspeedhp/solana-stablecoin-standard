use anchor_lang::prelude::*;
use crate::state::*;

declare_id!("8fsBJKMGbZbQUHAHLqzgY8vkAzmCHUYkwAEQ3AFNSqMR");

pub const TRANSFER_HOOK_ID: Pubkey = pubkey!("sssHook111111111111111111111111111111111111");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        preset: StablecoinPreset,
        name: String,
        symbol: String,
        uri: String,
        decimals: u8
    ) -> Result<()> {
        instructions::initialize::handler(ctx, preset, name, symbol, uri, decimals)
    }

    pub fn mint(ctx: Context<MintStablecoin>, amount: u64) -> Result<()> {
        instructions::mint::handler(ctx, amount)
    }

    pub fn burn(ctx: Context<BurnStablecoin>, amount: u64) -> Result<()> {
        instructions::burn::handler(ctx, amount)
    }

    pub fn pause(ctx: Context<ManagePause>) -> Result<()> {
        instructions::pause::pause_handler(ctx)
    }

    pub fn unpause(ctx: Context<ManagePause>) -> Result<()> {
        instructions::pause::unpause_handler(ctx)
    }

    pub fn freeze_account(ctx: Context<ManageFreeze>) -> Result<()> {
        instructions::freeze::freeze_handler(ctx)
    }

    pub fn thaw_account(ctx: Context<ManageFreeze>) -> Result<()> {
        instructions::freeze::thaw_handler(ctx)
    }

    pub fn add_role(ctx: Context<AddRole>, role_type: String, wallet: Pubkey) -> Result<()> {
        instructions::roles::add_handler(ctx, role_type, wallet)
    }

    pub fn remove_role(ctx: Context<RemoveRole>, role_type: String, wallet: Pubkey) -> Result<()> {
        instructions::roles::remove_handler(ctx, role_type, wallet)
    }

    pub fn blacklist_add(ctx: Context<ManageBlacklist>, user: Pubkey, reason: String) -> Result<()> {
        instructions::blacklist::add_handler(ctx, user, reason)
    }

    pub fn blacklist_remove(ctx: Context<ManageBlacklist>, user: Pubkey) -> Result<()> {
        instructions::blacklist::remove_handler(ctx, user)
    }

    pub fn seize(ctx: Context<SeizeStablecoin>, amount: u64) -> Result<()> {
        instructions::seize::handler(ctx, amount)
    }
}

pub mod error;
pub mod instructions;
pub mod state;
pub mod constants;

pub use instructions::*;
