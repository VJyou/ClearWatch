use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::RiskEntry;
use crate::error::ClearWatchError;

#[derive(Accounts)]
#[instruction(flagged_address: Pubkey, incident_type: String)]
pub struct ReportAddress<'info> {
    #[account(mut)]
    pub reporter: Signer<'info>,

    #[account(
        init,
        payer = reporter,
        space = RiskEntry::SPACE,
        seeds = [b"risk_entry", flagged_address.as_ref()],
        bump,
    )]
    pub risk_entry: Account<'info, RiskEntry>,

    /// CHECK: The vault PDA that holds staked SOL
    #[account(
        mut,
        seeds = [b"stake_vault", flagged_address.as_ref()],
        bump,
    )]
    pub stake_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ReportAddress>,
    flagged_address: Pubkey,
    incident_type: String,
) -> Result<()> {
    require!(incident_type.len() <= 64, ClearWatchError::IncidentTypeTooLong);

    let clock = Clock::get()?;
    let entry = &mut ctx.accounts.risk_entry;

    entry.address = flagged_address;
    entry.tier = 1;
    entry.incident_type = incident_type;
    entry.reporter = ctx.accounts.reporter.key();
    entry.stake_amount = RiskEntry::STAKE_AMOUNT;
    entry.timestamp = clock.unix_timestamp;
    entry.report_count = 1;
    entry.expires_at = clock.unix_timestamp + RiskEntry::TIER1_TTL;

    // Transfer 0.1 SOL stake to vault
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.key(),
            system_program::Transfer {
                from: ctx.accounts.reporter.to_account_info(),
                to: ctx.accounts.stake_vault.to_account_info(),
            },
        ),
        RiskEntry::STAKE_AMOUNT,
    )?;

    msg!(
        "ClearWatch: Address {} flagged as Tier 1 by {}. Expires at {}.",
        flagged_address,
        ctx.accounts.reporter.key(),
        entry.expires_at
    );

    Ok(())
}
