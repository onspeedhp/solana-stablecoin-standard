use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use spl_transfer_hook_interface::instruction::TransferHookInstruction;
use spl_tlv_account_resolution::{
    state::ExtraAccountMetaList,
    account_meta::ExtraAccountMeta,
    seeds::Seed,
};
use stablecoin::state::*;

declare_id!("sssHook111111111111111111111111111111111111");

pub mod stablecoin_program {
    use super::*;
    declare_id!("8fsBJKMGbZbQUHAHLqzgY8vkAzmCHUYkwAEQ3AFNSqMR");
}

#[program]
pub mod transfer_hook {
    use super::*;

    pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        let account_metas = vec![
            // Index 5: StablecoinConfig
            ExtraAccountMeta::new_with_seeds(
                &[Seed::Literal(b"config".to_vec()), Seed::AccountKey(1)], // Seed from Mint (Account 1)
                false, // is_signer
                false  // is_writable
            )?,
            // Index 6: PauseState
            ExtraAccountMeta::new_with_seeds(
                &[Seed::Literal(b"pause".to_vec()), Seed::AccountData(5, 45, 32)], // From Config (offset to mint is 45 approx)
                false,
                false
            )?,
            // Index 7: Blacklist for source owner
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal(b"blacklist".to_vec()),
                    Seed::AccountData(5, 45, 32), // From Config key (via Mint in Config offset)
                    Seed::AccountData(0, 32, 32) // From Source Token Account Owner (Offset 32)
                ],
                false,
                false
            )?,
            // Index 8: Blacklist for destination owner
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal(b"blacklist".to_vec()),
                    Seed::AccountData(5, 45, 32),
                    Seed::AccountData(2, 32, 32) // From Destination Token Account Owner (Offset 32)
                ],
                false,
                false
            )?,
        ];

        // The metadata account is the PDA derived from [b"extra-account-metas", mint]
        ExtraAccountMetaList::init::<TransferHookInstruction>(
            &mut ctx.accounts.extra_metas_account.try_borrow_mut_data()?,
            &account_metas
        )?;

        Ok(())
    }

    pub fn execute(ctx: Context<Execute>, _amount: u64) -> Result<()> {
        // Deserialize accounts safely using Anchor
        // We use UncheckedAccount in Execute struct to allow flexible resolution, 
        // but we deserialize here for safety.

        // 1. Check Global Pause
        let pause_data = ctx.accounts.pause_state.try_borrow_data()?;
        let pause_state = PauseState::try_deserialize(&mut &pause_data[..])?;
        if pause_state.is_paused {
            return err!(stablecoin::error::StablecoinError::Paused);
        }

        // 2. Check Blacklist for source owner
        if !ctx.accounts.source_blacklist.data_is_empty() {
            let data = ctx.accounts.source_blacklist.try_borrow_data()?;
            let blacklist = BlacklistAccount::try_deserialize(&mut &data[..])?;
            if blacklist.is_blacklisted {
                return err!(stablecoin::error::StablecoinError::Blacklisted);
            }
        }

        // 3. Check Blacklist for destination owner
        if !ctx.accounts.destination_blacklist.data_is_empty() {
            let data = ctx.accounts.destination_blacklist.try_borrow_data()?;
            let blacklist = BlacklistAccount::try_deserialize(&mut &data[..])?;
            if blacklist.is_blacklisted {
                return err!(stablecoin::error::StablecoinError::Blacklisted);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: ExtraAccountMetaList PDA
    #[account(
        init,
        payer = payer,
        space = 8 + 4 + (4 * 35),
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_metas_account: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(token::mint = mint)]
    pub source: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: Owner
    pub owner: UncheckedAccount<'info>,
    /// CHECK: Extra metas
    pub extra_metas: UncheckedAccount<'info>,
    
    /// The following are provided by the extra metas resolution
    /// CHECK: Stablecoin Config PDA from main program
    #[account(owner = stablecoin_program::ID)]
    pub config: UncheckedAccount<'info>,
    /// CHECK: Pause State PDA
    #[account(owner = stablecoin_program::ID)]
    pub pause_state: UncheckedAccount<'info>,
    /// CHECK: Blacklist PDA for source
    #[account(owner = stablecoin_program::ID)]
    pub source_blacklist: UncheckedAccount<'info>,
    /// CHECK: Blacklist PDA for destination
    #[account(owner = stablecoin_program::ID)]
    pub destination_blacklist: UncheckedAccount<'info>,
}
