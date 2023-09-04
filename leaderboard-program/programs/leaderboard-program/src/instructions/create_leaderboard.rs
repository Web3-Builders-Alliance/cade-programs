use anchor_lang::prelude::*;

use crate::state::{game::*, leaderboard::*};

// Create an instance of "Leaderboard"
// ** This instruction should be called via CPI from
// ** an external game program
pub fn create_leaderboard(
    ctx: Context<CreateLeaderboard>, 
    game_id: Game::AvailableGames, 
) -> Result<()> {
    ctx.accounts.game.set_inner(
        Leaderboard::new(
            game_id,
            ctx.accounts.game
        )
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreateLeaderboard<'info> {
    #[account(
        init, 
        space = Leaderboard::SPACE,
        payer = authority, 
        seeds = [
            Leaderboard::SEED_PREFIX.as_bytes(),
            game_program.key().as_ref(),
            game_id.to_le_bytes().as_ref(),
        ], 
        bump, 
    )]
    pub leaderboard: Account<'info, Leaderboard>,
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub game_program: Program<'info>,
    pub system_program: Program<'info, System>
}