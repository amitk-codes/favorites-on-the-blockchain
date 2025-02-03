use anchor_lang::prelude::*;

use crate::{Favorites, ANCHOR_DISCRIMINATOR};

pub fn set_favorites_handler(
    ctx: Context<SetFavorites>,
    number: u64,
    color: String,
    hobbies: Vec<String>,
) -> Result<()> {
    msg!(
        "Calling 'set_favorites' function with program_id: {}",
        ctx.program_id
    );

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

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
