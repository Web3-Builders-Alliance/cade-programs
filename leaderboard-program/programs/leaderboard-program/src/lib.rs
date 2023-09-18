use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

pub use state::*;
pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod leaderboard_program {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, game_id: AvailableGames, score: u64) -> Result<()> {
        instructions::create_game(ctx, game_id, score)
    }

    pub fn create_leaderboard(ctx: Context<CreateLeaderboard>, game_id: AvailableGames, player: Player) -> Result<()> {
        instructions::create_leaderboard(ctx, game_id, player)
    }
}
