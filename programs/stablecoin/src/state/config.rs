use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StablecoinConfig {
    #[max_len(32)]
    pub name: String,
    #[max_len(10)]
    pub symbol: String,
    #[max_len(100)]
    pub uri: String,
    pub decimals: u8,

    pub enable_transfer_hook: bool,
    pub enable_permanent_delegate: bool,
    pub default_account_frozen: bool,

    pub master_authority: Pubkey,
    pub mint: Pubkey,
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
    pub wallet: Pubkey,
    #[max_len(20)]
    pub role_type: String,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BlacklistAccount {
    pub wallet: Pubkey,
    #[max_len(50)]
    pub reason: String,
    pub created_at: i64,
    pub is_blacklisted: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PauseState {
    pub is_paused: bool,
    pub bump: u8,
}

pub mod role_types {
    pub const MASTER: &str = "master";
    pub const MINTER: &str = "minter";
    pub const BURNER: &str = "burner";
    pub const PAUSER: &str = "pauser";
    pub const FREEZER: &str = "freezer";
    pub const BLACKLISTER: &str = "blacklister";
    pub const SEIZER: &str = "seizer";
}
