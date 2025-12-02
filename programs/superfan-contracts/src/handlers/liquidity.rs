use anchor_lang::prelude::*;

use crate::contexts::*;
use crate::errors::SuperfanError;

pub fn fund_pool(
    ctx: Context<FundPool>,
    capacity: u64,
    first_n_limit: u16,
) -> Result<()> {
    require!(capacity > 0, SuperfanError::InvalidAmount);
    let pool = &mut ctx.accounts.liquidity_pool;
    pool.sponsor = ctx.accounts.sponsor.key();
    pool.capacity = capacity;
    pool.locked = 0;
    pool.first_n_limit = first_n_limit;
    pool.bump = ctx.bumps.liquidity_pool;
    Ok(())
}

pub fn withdraw_pool(_ctx: Context<WithdrawPool>) -> Result<()> {
    // Placeholder: token movements to be implemented with a proper vault.
    // Require that nothing is locked before withdrawal.
    // let pool = &ctx.accounts.liquidity_pool;
    // require!(pool.locked == 0, SuperfanError::InvalidStatus);
    Ok(())
}
