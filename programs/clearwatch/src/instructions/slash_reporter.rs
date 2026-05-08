use anchor_lang::prelude::*;
use crate::state::RiskEntry;
use crate::error::ClearWatchError;

#[derive(Accounts)]
#[instruction(flagged_address: Pubkey)]
pub struct SlashReporter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [b"risk_entry", flagged_address.as_ref()],
        bump,
    )]
    pub risk_entry: Account<'info, RiskEntry>,

    /// CHECK: The vault PDA that holds staked SOL — slashed to authority
    #[account(
        mut,
        seeds = [b"stake_vault", flagged_address.as_ref()],
        bump,
    )]
    pub stake_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SlashReporter>,
    flagged_address: Pubkey,
    vault_bump: u8,
) -> Result<()> {
    let vault = &ctx.accounts.stake_vault;
    let vault_balance = vault.lamports();

    if vault_balance > 0 {
        // Transfer slashed stake from vault to authority (governance / DAO)
        let seeds = &[
            b"stake_vault",
            flagged_address.as_ref(),
            &[vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.key(),
                anchor_lang::system_program::Transfer {
                    from: vault.to_account_info(),
                    to: ctx.accounts.authority.to_account_info(),
                },
                signer_seeds,
            ),
            vault_balance,
        )?;

        msg!(
            "ClearWatch: Reporter {} slashed {} lamports for false report on {}",
            ctx.accounts.risk_entry.reporter,
            vault_balance,
            flagged_address
        );
    }

    // risk_entry is closed automatically via `close = authority`
    Ok(())
}
