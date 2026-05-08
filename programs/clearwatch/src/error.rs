use anchor_lang::prelude::*;

#[error_code]
pub enum ClearWatchError {
    #[msg("Insufficient stake amount. Required: 0.1 SOL")]
    InsufficientStake,
    #[msg("Address is already flagged at this tier or higher")]
    AlreadyFlagged,
    #[msg("Risk entry has expired")]
    EntryExpired,
    #[msg("Unauthorized: only the original reporter can perform this action")]
    UnauthorizedReporter,
    #[msg("Cannot upgrade to a lower or equal tier")]
    InvalidTierUpgrade,
    #[msg("Incident type string too long (max 64 chars)")]
    IncidentTypeTooLong,
    #[msg("Purpose string too long (max 256 chars)")]
    PurposeTooLong,
}
