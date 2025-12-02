use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod contexts;
pub mod handlers;
pub mod state;

pub use crate::errors::*;
pub use crate::events::*;
pub use crate::contexts::*;
pub use crate::state::*;

declare_id!("Cdwbw2aAasToUrUkmnj6UzECZ46wHczLbiKsmir2Xhc7");

#[program]
pub mod superfan_contracts {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        max_sponsors: u32,
        usdc_mint: Pubkey,
        admin: Pubkey,
    ) -> Result<()> {
        handlers::config::initialize_config(ctx, max_sponsors, usdc_mint, admin)
    }

    pub fn register_sponsor(
        ctx: Context<RegisterSponsor>,
        name_hash: [u8; 32],
    ) -> Result<()> {
        handlers::config::register_sponsor(ctx, name_hash)
    }

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
        handlers::market::create_market(
            ctx,
            market_id,
            artist_wallet,
            artist_id_hash,
            trading_starts_at,
            trading_ends_at,
            resolution_deadline,
            conviction_threshold_bps,
            max_pool_exposure,
            liquidity_pool,
            signal_oracle,
        )
    }

    pub fn lock_market(ctx: Context<UpdateMarketStatus>) -> Result<()> {
        handlers::market::lock_market(ctx)
    }

    pub fn cancel_market(ctx: Context<UpdateMarketStatus>) -> Result<()> {
        handlers::market::cancel_market(ctx)
    }

    pub fn resolve_market(
        ctx: Context<UpdateMarketStatus>,
        outcome_yes: bool,
    ) -> Result<()> {
        handlers::market::resolve_market(ctx, outcome_yes)
    }

    pub fn fund_pool(
        ctx: Context<FundPool>,
        capacity: u64,
        first_n_limit: u16,
    ) -> Result<()> {
        handlers::liquidity::fund_pool(ctx, capacity, first_n_limit)
    }

    pub fn withdraw_pool(ctx: Context<WithdrawPool>) -> Result<()> {
        handlers::liquidity::withdraw_pool(ctx)
    }

    pub fn open_position(ctx: Context<OpenPosition>, amount: u64) -> Result<()> {
        handlers::positions::open_position(ctx, amount)
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        handlers::positions::close_position(ctx)
    }

    pub fn update_reputation(
        ctx: Context<UpdateReputation>,
        delta: i64,
    ) -> Result<()> {
        handlers::reputation::update_reputation(ctx, delta)
    }

    pub fn distribute_payouts(ctx: Context<DistributePayouts>) -> Result<()> {
        handlers::payouts::distribute_payouts(ctx)
    }

    pub fn submit_signal_commitment(
        ctx: Context<SubmitSignalCommitment>,
        commitment_root: [u8; 32],
    ) -> Result<()> {
        handlers::signal::submit_signal_commitment(ctx, commitment_root)
    }
}
