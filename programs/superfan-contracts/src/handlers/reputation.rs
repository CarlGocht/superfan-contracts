use anchor_lang::prelude::*;

use crate::contexts::*;
use crate::errors::SuperfanError;
use crate::events::ReputationUpdated;

pub fn update_reputation(ctx: Context<UpdateReputation>, delta: i64) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(
        ctx.accounts.admin.key() == config.admin,
        SuperfanError::Unauthorized
    );
    let registry = &mut ctx.accounts.scout_registry;
    registry.user = ctx.accounts.user.key();
    registry.score = registry
        .score
        .checked_add(delta)
        .ok_or(SuperfanError::MathOverflow)?;
    emit!(ReputationUpdated {
        user: registry.user,
        new_score: registry.score,
    });
    Ok(())
}
