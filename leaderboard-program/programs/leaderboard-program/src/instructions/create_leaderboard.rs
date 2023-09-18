use anchor_lang::prelude::*;

use crate::{state::{game::*, leaderboard::*}, Player};

// Create an instance of "Leaderboard"
// ** This instruction should be called via CPI from
// ** an external game program
pub fn create_leaderboard(
    ctx: Context<CreateLeaderboard>, 
    game_id: AvailableGames, 
    player: Player
) -> Result<()> {
    ctx.accounts.leaderboard.set_inner(
        Leaderboard::new_with_game(
            game_id,
            player,
            *ctx.bumps
                .get("leaderboard")
                .expect("Failed to derive bump for `leaderboard`"),
        )?
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    game_id: AvailableGames,
)]
pub struct CreateLeaderboard<'info> {
    #[account(
        init, 
        space = Leaderboard::SPACE,
        payer = authority, 
        seeds = [
            Leaderboard::SEED_PREFIX.as_bytes(),
            game_program.key().as_ref(),
        ], 
        bump, 
    )]
    pub leaderboard: Account<'info, Leaderboard>,
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is safe because we don't write to this account.
    pub game_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}