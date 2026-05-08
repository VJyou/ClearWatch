use anchor_lang::prelude::*;
use crate::state::{RiskEntry, evaluate_risk};

/// Read-only risk gate: returns (is_clear, risk_score, risk_tier_at_check)
/// without writing an InnocenceProof PDA. Designed for high-throughput
/// callers (DEX aggregators, bridges) that pay compute units per swap and
/// don't need a per-transaction audit trail.
#[derive(Accounts)]
#[instruction(counterparty: Pubkey)]
pub struct CheckOnly<'info> {
    pub caller: Signer<'info>,

    #[account(
        seeds = [b"risk_entry", counterparty.as_ref()],
        bump,
    )]
    pub risk_entry: Option<Account<'info, RiskEntry>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CheckOnlyResult {
    pub is_clear: bool,
    pub risk_score: u8,
    pub risk_tier_at_check: u8,
}

pub fn handler(
    ctx: Context<CheckOnly>,
    counterparty: Pubkey,
) -> Result<CheckOnlyResult> {
    let clock = Clock::get()?;
    let risk_entry_ref = ctx.accounts.risk_entry.as_ref().map(|a| &**a);
    let (is_clear, risk_score, risk_tier_at_check) =
        evaluate_risk(risk_entry_ref, clock.unix_timestamp);

    if is_clear {
        msg!(
            "ClearWatch check_only CLEAR: caller={}, counterparty={}",
            ctx.accounts.caller.key(),
            counterparty
        );
    } else {
        msg!(
            "ClearWatch check_only BLOCKED: caller={}, counterparty={} flagged at Tier {}",
            ctx.accounts.caller.key(),
            counterparty,
            risk_tier_at_check
        );
    }

    Ok(CheckOnlyResult {
        is_clear,
        risk_score,
        risk_tier_at_check,
    })
}
