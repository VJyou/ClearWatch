use anchor_lang::prelude::*;
use crate::state::{RiskEntry, InnocenceProof, evaluate_risk};
use crate::error::ClearWatchError;

#[derive(Accounts)]
#[instruction(counterparty: Pubkey, amount: u64, purpose: String)]
pub struct CheckAndProve<'info> {
    #[account(mut)]
    pub agent: Signer<'info>,

    /// Risk entry for the counterparty (optional — may not exist)
    #[account(
        seeds = [b"risk_entry", counterparty.as_ref()],
        bump,
    )]
    pub risk_entry: Option<Account<'info, RiskEntry>>,

    #[account(
        init,
        payer = agent,
        space = InnocenceProof::SPACE,
        seeds = [b"innocence_proof", agent.key().as_ref(), counterparty.as_ref()],
        bump,
    )]
    pub innocence_proof: Account<'info, InnocenceProof>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CheckAndProve>,
    counterparty: Pubkey,
    amount: u64,
    purpose: String,
) -> Result<()> {
    require!(purpose.len() <= 256, ClearWatchError::PurposeTooLong);

    let clock = Clock::get()?;
    let risk_entry_ref = ctx.accounts.risk_entry.as_ref().map(|a| &**a);
    let (is_clear, risk_score, risk_tier_at_check) =
        evaluate_risk(risk_entry_ref, clock.unix_timestamp);

    let purpose_hash = hash_purpose(&purpose);
    let proof_hash = compute_proof_hash(
        &ctx.accounts.agent.key(),
        &counterparty,
        amount,
        &purpose_hash,
        is_clear,
        clock.unix_timestamp,
    );

    let proof = &mut ctx.accounts.innocence_proof;
    proof.agent = ctx.accounts.agent.key();
    proof.counterparty = counterparty;
    proof.amount = amount;
    proof.purpose_hash = purpose_hash;
    proof.is_clear = is_clear;
    proof.risk_score = risk_score;
    proof.risk_tier_at_check = risk_tier_at_check;
    proof.timestamp = clock.unix_timestamp;
    proof.proof_hash = proof_hash;

    if is_clear {
        msg!(
            "ClearWatch CLEAR: agent={}, counterparty={}, amount={}, proof={:?}",
            proof.agent,
            counterparty,
            amount,
            proof_hash
        );
    } else {
        msg!(
            "ClearWatch BLOCKED: agent={}, counterparty={} is flagged at Tier {}",
            proof.agent,
            counterparty,
            risk_tier_at_check
        );
    }

    Ok(())
}

fn hash_purpose(purpose: &str) -> [u8; 32] {
    sha2_hash(purpose.as_bytes())
}

fn compute_proof_hash(
    agent: &Pubkey,
    counterparty: &Pubkey,
    amount: u64,
    purpose_hash: &[u8; 32],
    is_clear: bool,
    timestamp: i64,
) -> [u8; 32] {
    let mut data = Vec::with_capacity(32 + 32 + 8 + 32 + 1 + 8);
    data.extend_from_slice(agent.as_ref());
    data.extend_from_slice(counterparty.as_ref());
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(purpose_hash);
    data.push(is_clear as u8);
    data.extend_from_slice(&timestamp.to_le_bytes());
    sha2_hash(&data)
}

fn sha2_hash(data: &[u8]) -> [u8; 32] {
    solana_sha256_hasher::hash(data).to_bytes()
}
