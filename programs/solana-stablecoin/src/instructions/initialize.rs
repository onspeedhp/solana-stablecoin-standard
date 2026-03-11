use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use crate::state::*;

#[derive(Accounts)]
#[instruction(preset: StablecoinPreset)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + StablecoinConfig::INIT_SPACE,
        seeds = [b"config", mint.key().as_ref()],
        bump
    )]
    pub config: Account<'info, StablecoinConfig>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = admin,
        mint::freeze_authority = admin,
        extensions::metadata_pointer::authority = admin,
        extensions::metadata_pointer::metadata_address = mint,
        // SSS-2 Extensions (enabled conditionally based on preset)
        extensions::permanent_delegate::delegate = admin.key(),
        extensions::transfer_hook::authority = admin.key(),
        extensions::transfer_hook::program_id = crate::id(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, preset: StablecoinPreset) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.mint = ctx.accounts.mint.key();
    config.preset = preset;
    config.bump = ctx.bumps.config;
    
    // Default authorities
    config.pause_authority = ctx.accounts.admin.key();
    config.blacklist_authority = ctx.accounts.admin.key();
    config.freeze_authority = ctx.accounts.admin.key();

    msg!("Stablecoin initialized with preset: {:?}", preset);
    Ok(())
}
