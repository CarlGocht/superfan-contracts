use anchor_lang::prelude::*;

use crate::contexts::*;
use crate::errors::SuperfanError;

pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    max_sponsors: u32,
    usdc_mint: Pubkey,
    admin: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = admin;
    config.usdc_mint = usdc_mint;
    config.max_sponsors = max_sponsors;
    config.bump = ctx.bumps.config;
    Ok(())
}

pub fn register_sponsor(ctx: Context<RegisterSponsor>, name_hash: [u8; 32]) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(
        ctx.accounts.authority.key() == config.admin,
        SuperfanError::Unauthorized
    );

    let sponsor = &mut ctx.accounts.sponsor;
    sponsor.authority = ctx.accounts.authority.key();
    sponsor.name_hash = name_hash;
    sponsor.bump = ctx.bumps.sponsor;

    let counter = &mut ctx.accounts.market_counter;
    counter.sponsor = sponsor.key();
    counter.next_market_id = 1;
    counter.bump = ctx.bumps.market_counter;

    Ok(())
}
