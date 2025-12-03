use anchor_lang::prelude::*;

use crate::state::{
    LiquidityPool, Market, Position, ScoutRegistry, SignalCommitment, Sponsor, SponsorMarketCounter,
    SuperfanConfig,
};

// Config + sponsor
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        seeds = [b"superfan_config"],
        bump,
        payer = payer,
        space = 8 + SuperfanConfig::SPACE
    )]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterSponsor<'info> {
    #[account(mut, seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"sponsor", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Sponsor::SPACE
    )]
    pub sponsor: Account<'info, Sponsor>,
    #[account(
        init,
        seeds = [b"market_counter", sponsor.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + SponsorMarketCounter::SPACE
    )]
    pub market_counter: Account<'info, SponsorMarketCounter>,
    pub system_program: Program<'info, System>,
}

// Market lifecycle
#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct CreateMarket<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"sponsor", authority.key().as_ref()],
        bump = sponsor.bump,
        has_one = authority
    )]
    pub sponsor: Account<'info, Sponsor>,
    #[account(
        mut,
        seeds = [b"market_counter", sponsor.key().as_ref()],
        bump = market_counter.bump
    )]
    pub market_counter: Account<'info, SponsorMarketCounter>,
    #[account(
        init,
        seeds = [b"market", sponsor.key().as_ref(), &market_id.to_le_bytes()],
        bump,
        payer = authority,
        space = 8 + Market::SPACE
    )]
    pub market: Account<'info, Market>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMarketStatus<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"sponsor", authority.key().as_ref()],
        bump = sponsor.bump,
        has_one = authority
    )]
    pub sponsor: Account<'info, Sponsor>,
    #[account(mut, seeds = [b"market", sponsor.key().as_ref(), &market.market_id.to_le_bytes()], bump = market.bump)]
    pub market: Account<'info, Market>,
}

// Liquidity
#[derive(Accounts)]
pub struct FundPool<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(address = config.usdc_mint)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(
        seeds = [b"sponsor", authority.key().as_ref()],
        bump = sponsor.bump,
        has_one = authority
    )]
    pub sponsor: Account<'info, Sponsor>,
    #[account(
        init,
        seeds = [b"liquidity_pool", sponsor.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + LiquidityPool::SPACE
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = liquidity_pool
    )]
    pub liquidity_vault: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub sponsor_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawPool<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(address = config.usdc_mint)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(
        seeds = [b"sponsor", authority.key().as_ref()],
        bump = sponsor.bump,
        has_one = authority
    )]
    pub sponsor: Account<'info, Sponsor>,
    #[account(mut, seeds = [b"liquidity_pool", sponsor.key().as_ref()], bump = liquidity_pool.bump)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = liquidity_pool
    )]
    pub liquidity_vault: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub sponsor_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}

// Positions
#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = config.usdc_mint)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(
        seeds = [b"market", market.sponsor.as_ref(), &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        constraint = liquidity_pool.key() == market.liquidity_pool @ crate::errors::SuperfanError::InvalidStatus
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = liquidity_pool
    )]
    pub liquidity_vault: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(
        init,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + Position::SPACE
    )]
    pub position: Account<'info, Position>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = config.usdc_mint)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(
        seeds = [b"market", market.sponsor.as_ref(), &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        constraint = liquidity_pool.key() == market.liquidity_pool @ crate::errors::SuperfanError::InvalidStatus
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = liquidity_pool
    )]
    pub liquidity_vault: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(
        mut,
        close = user,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump = position.bump,
        has_one = user,
        has_one = market
    )]
    pub position: Account<'info, Position>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}

// Reputation
#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: user identity tracked off-chain; signature not required here
    pub user: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"scout", user.key().as_ref()],
        bump,
        payer = admin,
        space = 8 + ScoutRegistry::SPACE
    )]
    pub scout_registry: Account<'info, ScoutRegistry>,
    pub system_program: Program<'info, System>,
}

// Payouts
#[derive(Accounts)]
pub struct DistributePayouts<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"market", market.sponsor.as_ref(), &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
}

// Signal commit
#[derive(Accounts)]
pub struct SubmitSignalCommitment<'info> {
    #[account(seeds = [b"superfan_config"], bump = config.bump)]
    pub config: Account<'info, SuperfanConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"sponsor", authority.key().as_ref()],
        bump = sponsor.bump,
        has_one = authority
    )]
    pub sponsor: Account<'info, Sponsor>,
    #[account(
        seeds = [b"market", sponsor.key().as_ref(), &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
    #[account(
        init,
        seeds = [b"signal", market.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + SignalCommitment::SPACE
    )]
    pub signal_commit: Account<'info, SignalCommitment>,
    pub system_program: Program<'info, System>,
}
