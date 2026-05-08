pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FDMGN1Gp62gK1TAnVvq2DM4HV6BhFwJ9Me5djLVKEKgB");

#[program]
pub mod clearwatch {
    use super::*;

    pub fn report_address(
        ctx: Context<ReportAddress>,
        flagged_address: Pubkey,
        incident_type: String,
    ) -> Result<()> {
        instructions::report_address::handler(ctx, flagged_address, incident_type)
    }

    pub fn check_and_prove(
        ctx: Context<CheckAndProve>,
        counterparty: Pubkey,
        amount: u64,
        purpose: String,
    ) -> Result<()> {
        instructions::check_and_prove::handler(ctx, counterparty, amount, purpose)
    }

    pub fn upgrade_tier(
        ctx: Context<UpgradeTier>,
        flagged_address: Pubkey,
        new_tier: u8,
    ) -> Result<()> {
        instructions::upgrade_tier::handler(ctx, flagged_address, new_tier)
    }

    pub fn slash_reporter(
        ctx: Context<SlashReporter>,
        flagged_address: Pubkey,
        vault_bump: u8,
    ) -> Result<()> {
        instructions::slash_reporter::handler(ctx, flagged_address, vault_bump)
    }
}
