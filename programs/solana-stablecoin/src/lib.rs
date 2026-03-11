pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8fsBJKMGbZbQUHAHLqzgY8vkAzmCHUYkwAEQ3AFNSqMR");

#[program]
pub mod solana_stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, preset: StablecoinPreset) -> Result<()> {
        instructions::initialize::handler(ctx, preset)
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

    pub fn update_roles(ctx: Context<UpdateRoles>, user: Pubkey, roles: u16) -> Result<()> {
        instructions::roles::handler(ctx, user, roles)
    }

    pub fn blacklist_add(ctx: Context<ManageBlacklist>, user: Pubkey) -> Result<()> {
        instructions::blacklist::add_handler(ctx, user)
    }

    pub fn blacklist_remove(ctx: Context<ManageBlacklist>, user: Pubkey) -> Result<()> {
        instructions::blacklist::remove_handler(ctx, user)
    }

    pub fn seize(ctx: Context<SeizeStablecoin>, amount: u64) -> Result<()> {
        instructions::seize::handler(ctx, amount)
    }

    pub fn transfer_hook(ctx: Context<Execute>, amount: u64) -> Result<()> {
        instructions::transfer_hook::handler(ctx, amount)
    }
}
