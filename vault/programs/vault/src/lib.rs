use anchor_lang::prelude::*;

declare_id!("BEyfpBKR2zcdXhuKoECkiWdkosXq9bR9xFTrytcG6Y6k");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
