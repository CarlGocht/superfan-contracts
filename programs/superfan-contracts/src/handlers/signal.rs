use anchor_lang::prelude::*;

use crate::contexts::*;
use crate::events::SignalCommitted;

pub fn submit_signal_commitment(
    ctx: Context<SubmitSignalCommitment>,
    commitment_root: [u8; 32],
) -> Result<()> {
    let commit = &mut ctx.accounts.signal_commit;
    commit.market = ctx.accounts.market.key();
    commit.commitment_root = commitment_root;
    commit.recorded_at = Clock::get()?.unix_timestamp;
    commit.bump = ctx.bumps.signal_commit;
    emit!(SignalCommitted {
        market: commit.market,
        commitment_root,
        recorded_at: commit.recorded_at,
    });
    Ok(())
}
