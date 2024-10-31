use anchor_lang::prelude::*;

declare_id!("8Lw8tQWgYc64CCRxA9w4K36iTkhUYKpuHe56hU4wRTHi");

#[program]
pub mod favorites_on_the_blockchain {
    use super::*;
}

#[account]
#[derive(InitSpace)]
pub struct Favorites{
    pub number: u64,

    #[max_len(60)]
    pub color: String,

    #[max_len(5, 60)]
    pub hobbies: Vec<String>,
}