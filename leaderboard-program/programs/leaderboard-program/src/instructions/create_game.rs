use anchor_lang::prelude::*;

use crate::state::{game::*, player::*};

// Creates an instance of a "game"
// ** This instruction should be called via CPI from
// ** an external game program.
pub fn create_game(
    ctx: Context<CreateGame>, 
    game_id: Game::AvailableGames, 
    score: u64,
) -> Result<()> {
    ctx.accounts.game.set_inner(
        Game::new(
            ctx.accounts.player.key(),
            game_id,
            ctx.accounts.game_program.key(),
            score,
        )
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        space = Game::SPACE,
        payer = player, 
        seeds = [
            Game::SEED_PREFIX.as_bytes(),
            game_program.key().as_ref(),
            game_id.to_le_bytes().as_ref(),
        ], 
        bump, 
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub game_program: Program<'info>,
    pub system_program: Program<'info, System>
}