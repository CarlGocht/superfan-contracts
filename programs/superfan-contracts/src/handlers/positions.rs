use anchor_lang::prelude::*;

use crate::contexts::*;
use crate::errors::SuperfanError;
use crate::events::PositionOpened;

pub fn open_position(ctx: Context<OpenPosition>, amount: u64) -> Result<()> {
    require!(amount > 0, SuperfanError::InvalidAmount);
    let now = Clock::get()?.unix_timestamp;
    let market = &ctx.accounts.market;
    require!(
        market.status == crate::state::MarketStatus::Pending as u8,
        SuperfanError::InvalidStatus
    );
    require!(now >= market.trading_starts_at, SuperfanError::TradingStillOpen);
    require!(now <= market.trading_ends_at, SuperfanError::TradingAlreadyStarted);

    let pool = &mut ctx.accounts.liquidity_pool;
    let new_locked = pool
        .locked
        .checked_add(amount)
        .ok_or(SuperfanError::MathOverflow)?;
    require!(new_locked <= pool.capacity, SuperfanError::InsufficientLiquidity);
    pool.locked = new_locked;

    let position = &mut ctx.accounts.position;
    position.user = ctx.accounts.user.key();
    position.market = ctx.accounts.market.key();
    position.amount = amount;
    position.created_at = Clock::get()?.unix_timestamp;
    position.bump = ctx.bumps.position;
    emit!(PositionOpened {
        market: position.market,
        user: position.user,
        amount,
    });
    Ok(())
}

pub fn close_position(_ctx: Context<ClosePosition>) -> Result<()> {
    // Placeholder: redemption logic to be added.
    // Consider decrementing locked liquidity and transferring funds based on outcome.
    Ok(())
}
