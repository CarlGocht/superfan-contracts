use anchor_lang::prelude::*;

use crate::contexts::*;
use crate::errors::SuperfanError;
use crate::events::MarketResolved;
use crate::state::MarketStatus;

#[allow(clippy::too_many_arguments)]
pub fn create_market(
    ctx: Context<CreateMarket>,
    market_id: u64,
    artist_wallet: Pubkey,
    artist_id_hash: [u8; 32],
    trading_starts_at: i64,
    trading_ends_at: i64,
    resolution_deadline: i64,
    conviction_threshold_bps: u16,
    max_pool_exposure: u64,
    liquidity_pool: Pubkey,
    signal_oracle: Pubkey,
) -> Result<()> {
    require!(
        trading_ends_at > trading_starts_at,
        SuperfanError::InvalidTradingWindow
    );
    require!(
        resolution_deadline > trading_ends_at,
        SuperfanError::InvalidResolutionDeadline
    );

    let counter = &mut ctx.accounts.market_counter;
    require!(market_id == counter.next_market_id, SuperfanError::InvalidMarketId);
    counter.next_market_id = counter
        .next_market_id
        .checked_add(1)
        .ok_or(SuperfanError::MathOverflow)?;

    let market = &mut ctx.accounts.market;
    market.sponsor = ctx.accounts.sponsor.key();
    market.market_id = market_id;
    market.artist_wallet = artist_wallet;
    market.artist_id_hash = artist_id_hash;
    market.trading_starts_at = trading_starts_at;
    market.trading_ends_at = trading_ends_at;
    market.resolution_deadline = resolution_deadline;
    market.conviction_threshold_bps = conviction_threshold_bps;
    market.max_pool_exposure = max_pool_exposure;
    market.liquidity_pool = liquidity_pool;
    market.signal_oracle = signal_oracle;
    market.status = MarketStatus::Pending as u8;
    market.outcome = 0;
    market.resolved_at = 0;
    market.bump = ctx.bumps.market;

    Ok(())
}

pub fn lock_market(ctx: Context<UpdateMarketStatus>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let now = Clock::get()?.unix_timestamp;
    require!(
        market.status == MarketStatus::Pending as u8,
        SuperfanError::InvalidStatus
    );
    require!(now > market.trading_ends_at, SuperfanError::TradingStillOpen);
    market.status = MarketStatus::Locked as u8;
    Ok(())
}

pub fn cancel_market(ctx: Context<UpdateMarketStatus>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let now = Clock::get()?.unix_timestamp;
    require!(
        market.status == MarketStatus::Pending as u8,
        SuperfanError::InvalidStatus
    );
    require!(now < market.trading_starts_at, SuperfanError::TradingAlreadyStarted);
    market.status = MarketStatus::Cancelled as u8;
    Ok(())
}

pub fn resolve_market(ctx: Context<UpdateMarketStatus>, outcome_yes: bool) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let now = Clock::get()?.unix_timestamp;

    require!(
        market.status != MarketStatus::Cancelled as u8,
        SuperfanError::InvalidStatus
    );
    require!(
        market.status != MarketStatus::Resolved as u8,
        SuperfanError::InvalidStatus
    );
    require!(now >= market.trading_ends_at, SuperfanError::TradingStillOpen);
    require!(
        now <= market.resolution_deadline,
        SuperfanError::ResolutionDeadlinePassed
    );

    market.status = MarketStatus::Resolved as u8;
    market.resolved_at = now;
    market.outcome = if outcome_yes { 1 } else { 2 };

    emit!(MarketResolved {
        market: market.key(),
        sponsor: market.sponsor,
        outcome_yes,
        resolved_at: now,
    });

    Ok(())
}
