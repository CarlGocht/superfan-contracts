use anchor_lang::prelude::*;

#[event]
pub struct MarketResolved {
    pub market: Pubkey,
    pub sponsor: Pubkey,
    pub outcome_yes: bool,
    pub resolved_at: i64,
}

#[event]
pub struct PositionOpened {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ReputationUpdated {
    pub user: Pubkey,
    pub new_score: i64,
}

#[event]
pub struct SignalCommitted {
    pub market: Pubkey,
    pub commitment_root: [u8; 32],
    pub recorded_at: i64,
}
