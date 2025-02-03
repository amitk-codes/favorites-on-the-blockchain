use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(60)]
    pub color: String,

    #[max_len(5, 60)]
    pub hobbies: Vec<String>,
}
