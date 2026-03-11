use anchor_lang::prelude::*;

#[error_code]
pub enum StablecoinError {
    #[msg("Unauthorized: Missing required role")]
    Unauthorized,
    #[msg("Global pause is active")]
    Paused,
    #[msg("Account is blacklisted")]
    Blacklisted,
    #[msg("Invalid preset configuration")]
    InvalidPreset,
    #[msg("Overflow in calculation")]
    Overflow,
}
