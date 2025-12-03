use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

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

    // Transfer stake into the pool vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.liquidity_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

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

pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
    // Placeholder: redemption/payout logic. For now, refund staked amount.
    let position = &ctx.accounts.position;
    let amount = position.amount;
    let pool = &mut ctx.accounts.liquidity_pool;
    pool.locked = pool.locked.saturating_sub(amount);

    // transfer back to user from vault, using pool as authority
    let seeds: &[&[u8]] = &[
        b"liquidity_pool",
        ctx.accounts.liquidity_pool.sponsor.as_ref(),
        &[ctx.accounts.liquidity_pool.bump],
    ];
    let signer = &[seeds];
    let cpi_accounts = Transfer {
        from: ctx.accounts.liquidity_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.liquidity_pool.to_account_info(),
    };
    let cpi_ctx =
        CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}
