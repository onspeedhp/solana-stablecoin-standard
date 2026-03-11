use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint},
    token_2022_extensions::{
        metadata_pointer_initialize,
        token_metadata_initialize,
        permanent_delegate_initialize,
        transfer_hook_initialize,
        default_account_state_initialize,
    },
};
use anchor_spl::token_2022::spl_token_2022::state::AccountState;
use crate::state::*;

#[derive(Accounts)]
#[instruction(preset: StablecoinPreset, name: String, symbol: String, uri: String, decimals: u8)]
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
        space = 8 + RoleAccount::INIT_SPACE,
        seeds = [b"role", role_types::MASTER.as_bytes(), admin.key().as_ref()],
        bump
    )]
    pub admin_role: Account<'info, RoleAccount>,

    #[account(mut)]
    pub mint: Signer<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Initialize>, 
    preset: StablecoinPreset,
    name: String,
    symbol: String,
    uri: String,
    decimals: u8
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.name = name.clone();
    config.symbol = symbol.clone();
    config.uri = uri.clone();
    config.decimals = decimals;
    config.mint = ctx.accounts.mint.key();
    config.master_authority = ctx.accounts.admin.key();
    config.preset = preset;
    config.bump = ctx.bumps.config;

    // Features based on preset
    config.enable_transfer_hook = preset == StablecoinPreset::SSS2;
    config.enable_permanent_delegate = preset == StablecoinPreset::SSS2;
    config.default_account_frozen = preset == StablecoinPreset::SSS2;

    // RBAC: MASTER role
    let admin_role = &mut ctx.accounts.admin_role;
    admin_role.wallet = ctx.accounts.admin.key();
    admin_role.role_type = role_types::MASTER.to_string();
    admin_role.bump = ctx.bumps.admin_role;

    // CPI to initialize extensions on Mint
    // Metadata Pointer
    metadata_pointer_initialize(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_2022_extensions::MetadataPointerInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        Some(ctx.accounts.admin.key()),
        Some(ctx.accounts.mint.key()),
    )?;

    // Permanent Delegate
    if config.enable_permanent_delegate {
        permanent_delegate_initialize(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token_2022_extensions::PermanentDelegateInitialize {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
            ),
            &ctx.accounts.admin.key(),
        )?;
    }

    // Transfer Hook
    if config.enable_transfer_hook {
        // We assume the hook program ID is known/passed or constant. 
        // For SSS, let's use a placeholder or derived ID.
        // The user said: "The stablecoin program must only configure the mint with the transfer hook program ID."
        transfer_hook_initialize(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token_2022_extensions::TransferHookInitialize {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
            ),
            Some(ctx.accounts.admin.key()),
            Some(crate::TRANSFER_HOOK_ID), // This needs to be defined
        )?;
    }

    // Default Account State
    if config.default_account_frozen {
        default_account_state_initialize(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token_2022_extensions::DefaultAccountStateInitialize {
                    token_program_id: ctx.accounts.token_program.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
            ),
            &AccountState::Frozen,
        )?;
    }

    // Finally initialize the mint itself
    anchor_spl::token_interface::initialize_mint2(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_interface::InitializeMint2 {
                mint: ctx.accounts.mint.to_account_info(),
            },
        ),
        decimals,
        &ctx.accounts.admin.key(),
        Some(&ctx.accounts.admin.key()),
    )?;

    // Initialize Metadata fields
    token_metadata_initialize(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_2022_extensions::TokenMetadataInitialize {
                program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(), // Mint is the metadata account
                update_authority: ctx.accounts.admin.to_account_info(),
                mint_authority: ctx.accounts.admin.to_account_info(),
            },
        ),
        name,
        symbol,
        uri,
    )?;

    Ok(())
}
