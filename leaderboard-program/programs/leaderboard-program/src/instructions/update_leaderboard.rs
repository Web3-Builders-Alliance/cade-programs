use anchor_lang::prelude::*;

use crate::{state::{game::*, leaderboard::*}, Player};

pub fn update_leaderboard(
    ctx: Context<UpdateLeaderboard>,
    game_id: AvailableGames,
    player: Player,
) -> Result<()> {
    ctx.accounts.leaderboard.set_inner(
        Leaderboard::add_score(
            player
        )
    );
}

#[derive(Accounts)]
pub struct UpdateLeaderboard<'info> {
    #[account(
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