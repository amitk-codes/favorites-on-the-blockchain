use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod constant;
pub use constant::*;

pub mod instructions;
pub use instructions::*;

declare_id!("8Lw8tQWgYc64CCRxA9w4K36iTkhUYKpuHe56hU4wRTHi");

#[program]
pub mod favorites_on_the_blockchain {
    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        set_favorites_handler(ctx, number, color, hobbies)?;
        Ok(())
    }
}
