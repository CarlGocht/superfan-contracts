use anchor_lang::prelude::*;

#[error_code]
pub enum SuperfanError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Trading start cannot be in the past")]
    TradingStartsInPast,
    #[msg("Trading end must be after start")]
    InvalidTradingWindow,
    #[msg("Resolution deadline must be after trading end")]
    InvalidResolutionDeadline,
    #[msg("Market id does not match counter")]
    InvalidMarketId,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Trading window still open")]
    TradingStillOpen,
    #[msg("Trading already started; cannot cancel")]
    TradingAlreadyStarted,
    #[msg("Invalid market status for this action")]
    InvalidStatus,
    #[msg("Resolution deadline passed")]
    ResolutionDeadlinePassed,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
}
