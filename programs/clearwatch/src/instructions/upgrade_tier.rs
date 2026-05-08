use anchor_lang::prelude::*;
use crate::state::RiskEntry;
use crate::error::ClearWatchError;

#[derive(Accounts)]
#[instruction(flagged_address: Pubkey, new_tier: u8)]
pub struct UpgradeTier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"risk_entry", flagged_address.as_ref()],
        bump,
    )]
    pub risk_entry: Account<'info, RiskEntry>,
}

pub fn handler(
    ctx: Context<UpgradeTier>,
    _flagged_address: Pubkey,
    new_tier: u8,
) -> Result<()> {
    let entry = &mut ctx.accounts.risk_entry;

    require!(new_tier > entry.tier, ClearWatchError::InvalidTierUpgrade);

    let old_tier = entry.tier;
    entry.tier = new_tier;
    entry.report_count += 1;

    // Tier 2+ entries do not expire
    if new_tier >= 2 {
        entry.expires_at = i64::MAX;
    }

    msg!(
        "ClearWatch: Address {} upgraded from Tier {} to Tier {} by {}",
        entry.address,
        old_tier,
        new_tier,
        ctx.accounts.authority.key()
    );

    Ok(())
}
