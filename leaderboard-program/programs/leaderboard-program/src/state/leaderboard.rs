use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Leaderboard {
    pub game_id: AvailableGames,
    pub game_ref: Game,
    pub timestamp: u64,
    pub players: Vec<Player>,
}

impl Leaderboard {
    pub const SEED_PREFIX: &'static str = "leaderboard";

    pub const SPACE: usize = 8 // Discriminator
    + 4                        // Enum
    + 4                        // u64
    + 32;                      // Vec<Player>

    pub fn new_with_game(game_id: AvailableGames, game_ref: Game) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp as u64;

        Ok(Self {   
            game_id,
            game_ref,
            vec![],
            current_time,
        })
    }

    pub fn add_score(player: Player) -> Result<()> {
        players.push(player);
        Ok(())
    }
}

impl Default for Leaderboard {
    fn default() -> Self {
        AvailableGames::Game1
    }
}