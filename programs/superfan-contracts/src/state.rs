use anchor_lang::prelude::*;

#[account]
pub struct SuperfanConfig {
    pub admin: Pubkey,
    pub usdc_mint: Pubkey,
    pub max_sponsors: u32,
    pub bump: u8,
}

impl SuperfanConfig {
    pub const SPACE: usize = 32 + 32 + 4 + 1;
}

#[account]
pub struct Sponsor {
    pub authority: Pubkey,
    pub name_hash: [u8; 32],
    pub bump: u8,
}

impl Sponsor {
    pub const SPACE: usize = 32 + 32 + 1;
}

#[account]
pub struct SponsorMarketCounter {
    pub sponsor: Pubkey,
    pub next_market_id: u64,
    pub bump: u8,
}

impl SponsorMarketCounter {
    pub const SPACE: usize = 32 + 8 + 1;
}

#[account]
pub struct Market {
    pub sponsor: Pubkey,
    pub market_id: u64,
    pub artist_wallet: Pubkey,
    pub artist_id_hash: [u8; 32],
    pub trading_starts_at: i64,
    pub trading_ends_at: i64,
    pub resolution_deadline: i64,
    pub conviction_threshold_bps: u16,
    pub max_pool_exposure: u64,
    pub liquidity_pool: Pubkey,
    pub signal_oracle: Pubkey,
    pub status: u8,
    pub outcome: u8, // 0 = unresolved, 1 = yes, 2 = no
    pub resolved_at: i64,
    pub bump: u8,
}

impl Market {
    pub const SPACE: usize = 32 + 8 + 32 + 32 + 8 + 8 + 8 + 2 + 8 + 32 + 32 + 1 + 1 + 1 + 8;
}

#[account]
pub struct LiquidityPool {
    pub sponsor: Pubkey,
    pub capacity: u64,
    pub locked: u64,
    pub first_n_limit: u16,
    pub vault: Pubkey,
    pub bump: u8,
}

impl LiquidityPool {
    pub const SPACE: usize = 32 + 8 + 8 + 2 + 32 + 1;
}

#[account]
pub struct Position {
    pub user: Pubkey,
    pub market: Pubkey,
    pub amount: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Position {
    pub const SPACE: usize = 32 + 32 + 8 + 8 + 1;
}

#[account]
pub struct ScoutRegistry {
    pub user: Pubkey,
    pub score: i64,
    pub bump: u8,
}

impl ScoutRegistry {
    pub const SPACE: usize = 32 + 8 + 1;
}

#[account]
pub struct SignalCommitment {
    pub market: Pubkey,
    pub commitment_root: [u8; 32],
    pub recorded_at: i64,
    pub bump: u8,
}

impl SignalCommitment {
    pub const SPACE: usize = 32 + 32 + 8 + 1;
}

#[repr(u8)]
pub enum MarketStatus {
    Pending = 0,
    Locked = 1,
    Resolved = 2,
    Cancelled = 3,
}
