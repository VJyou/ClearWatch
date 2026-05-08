use anchor_lang::prelude::*;

#[account]
pub struct RiskEntry {
    pub address: Pubkey,
    pub tier: u8,
    pub incident_type: String,
    pub reporter: Pubkey,
    pub stake_amount: u64,
    pub timestamp: i64,
    pub report_count: u8,
    pub expires_at: i64,
}

impl RiskEntry {
    // 8 discriminator + 32 address + 1 tier + 4+64 incident_type + 32 reporter
    // + 8 stake_amount + 8 timestamp + 1 report_count + 8 expires_at
    pub const SPACE: usize = 8 + 32 + 1 + (4 + 64) + 32 + 8 + 8 + 1 + 8;

    pub const TIER1_TTL: i64 = 3600; // 1 hour in seconds
    pub const STAKE_AMOUNT: u64 = 100_000_000; // 0.1 SOL in lamports
}

#[account]
pub struct InnocenceProof {
    pub agent: Pubkey,
    pub counterparty: Pubkey,
    pub amount: u64,
    pub purpose_hash: [u8; 32],
    pub is_clear: bool,
    pub risk_score: u8,
    pub risk_tier_at_check: u8,
    pub timestamp: i64,
    pub proof_hash: [u8; 32],
}

impl InnocenceProof {
    // 8 discriminator + 32 agent + 32 counterparty + 8 amount + 32 purpose_hash
    // + 1 is_clear + 1 risk_score + 1 risk_tier_at_check + 8 timestamp + 32 proof_hash
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 32 + 1 + 1 + 1 + 8 + 32;
}
