use anchor_lang::prelude::*;

use crate::contexts::*;

pub fn distribute_payouts(_ctx: Context<DistributePayouts>) -> Result<()> {
    // Placeholder: wire to token transfers + fee flows.
    // Expected checks: market resolved, locked funds available, and iterate positions.
    Ok(())
}
