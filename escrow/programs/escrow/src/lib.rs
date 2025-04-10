use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("Eb3UQsugB6AQzoH2h1vQk2a4ZTdGxf5MQjt3Pna4Jo6K");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
