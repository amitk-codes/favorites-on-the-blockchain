use anchor_lang::prelude::*;

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
        msg!("Calling 'set_favorites' function with program_id: {}", ctx.program_id);
        
        let color_clone = color.clone();
        let hobbies_clone = hobbies.clone();
        
        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        msg!("For user with public key {}, favorite number is set to {}, color is set to {}, and hobbies are set to {:?}", 
            ctx.accounts.user.key(), 
            number, 
            color_clone, 
            hobbies_clone
        );

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(60)]
    pub color: String,

    #[max_len(5, 60)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
