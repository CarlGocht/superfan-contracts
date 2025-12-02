use anchor_lang::prelude::*;

declare_id!("Cdwbw2aAasToUrUkmnj6UzECZ46wHczLbiKsmir2Xhc7");

#[program]
pub mod superfan_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
