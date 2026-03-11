use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::StablecoinError;

#[derive(Accounts)]
#[instruction(role_type: String, wallet: Pubkey)]
pub struct AddRole<'info> {
    #[account(mut)]
    pub master: Signer<'info>,

    #[account(
        seeds = [b"role", role_types::MASTER.as_bytes(), master.key().as_ref()],
        bump = master_role.bump
    )]
    pub master_role: Account<'info, RoleAccount>,

    #[account(
        init,
        payer = master,
        space = 8 + RoleAccount::INIT_SPACE,
        seeds = [b"role", role_type.as_bytes(), wallet.as_ref()],
        bump
    )]
    pub new_role_account: Account<'info, RoleAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(role_type: String, wallet: Pubkey)]
pub struct RemoveRole<'info> {
    #[account(mut)]
    pub master: Signer<'info>,

    #[account(
        seeds = [b"role", role_types::MASTER.as_bytes(), master.key().as_ref()],
        bump = master_role.bump
    )]
    pub master_role: Account<'info, RoleAccount>,

    #[account(
        mut,
        close = master,
        seeds = [b"role", role_type.as_bytes(), wallet.as_ref()],
        bump = role_account_to_remove.bump
    )]
    pub role_account_to_remove: Account<'info, RoleAccount>,

    pub system_program: Program<'info, System>,
}

pub fn add_handler(ctx: Context<AddRole>, role_type: String, wallet: Pubkey) -> Result<()> {
    let role_acc = &mut ctx.accounts.new_role_account;
    role_acc.wallet = wallet;
    role_acc.role_type = role_type;
    role_acc.bump = ctx.bumps.new_role_account;

    msg!("Role {} added to wallet {}", role_acc.role_type, wallet);
    Ok(())
}

pub fn remove_handler(ctx: Context<RemoveRole>, role_type: String, wallet: Pubkey) -> Result<()> {
    msg!("Role {} removed from wallet {}", role_type, wallet);
    Ok(())
}
