use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StablecoinConfig {
    pub mint: Pubkey,
    pub admin: Pubkey,
    pub pause_authority: Pubkey,
    pub blacklist_authority: Pubkey,
    pub freeze_authority: Pubkey,
    pub preset: StablecoinPreset,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum StablecoinPreset {
    SSS1, // Minimal
    SSS2, // Compliant
}

#[account]
#[derive(InitSpace)]
pub struct RoleAccount {
    pub user: Pubkey,
    pub roles: u16, // Bitmask for roles
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BlacklistAccount {
    pub user: Pubkey,
    pub is_blacklisted: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PauseState {
    pub is_paused: bool,
    pub bump: u8,
}

pub mod roles {
    pub const ADMIN: u16 = 1 << 0;
    pub const MINTER: u16 = 1 << 1;
    pub const BURNER: u16 = 1 << 2;
    pub const BLACK_LISTER: u16 = 1 << 3;
    pub const FREEZER: u16 = 1 << 4;
}
