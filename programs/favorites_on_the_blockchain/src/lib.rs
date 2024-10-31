use anchor_lang::prelude::*;

declare_id!("8Lw8tQWgYc64CCRxA9w4K36iTkhUYKpuHe56hU4wRTHi");

#[program]
pub mod favorites_on_the_blockchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
