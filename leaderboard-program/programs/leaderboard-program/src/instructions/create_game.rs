use anchor_lang::prelude::*;

use crate::state::{game::*, player::*};

// Creates an instance of "Game"
// ** This instruction should be called via CPI from
// ** an external game program.
pub fn create_game(
    ctx: Context<CreateGame>, 
    game_id: AvailableGames, 
    score: u64,
) -> Result<()> {
    ctx.accounts.game.set_inner(
        Game::new(
            ctx.accounts.player.key(),
            game_id,
            ctx.accounts.game_program.key(),
            score,
            *ctx.bumps
                .get("game")
                .expect("Failed to derive bump for `game`"),
        )?
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    game_id: AvailableGames,
    score: u64,
)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        space = Game::SPACE,
        payer = player, 
        seeds = [
            Game::SEED_PREFIX.as_bytes(),
            game_program.key().as_ref(),
            player.key().as_ref(),
            score.to_le_bytes().as_ref()
        ], 
        bump, 
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is safe because we don't write to this account.
    pub game_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}