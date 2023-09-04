use anchor_lang::prelude::*;

use crate::state::game::*;

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

    // Creates a new leaderboard once a new game is played
    // for the first time
    pub fn new_with_game(game_id: AvailableGames, game_ref: Game) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp as u64;

        Ok(Self {   
            game_id,
            game_ref,
            vec![],
            current_time,
        })
    }

    // Adds a score to the leaderboard
    pub fn add_score(player: Player) -> Result<()> {
        // *** Need a max length of 10 or else "players" will grow too large
        // 1. Check if "players" has hit max length
        // 2. Loop through vec and return position that player.score is more than
        // 3. If it's less than all of the current items, break function
        // 4. If a position is returned, push to vec at the returned position
        // *** The "players" field could also be an array of length 10 with sample scores
        // *** However, I will avoid this for now, as we don't have enough information about the games
        players.push(player);
        Ok(())
    }
}

// This is only for testing purposes
impl Default for Leaderboard {
    fn default() -> Self {
        AvailableGames::CoinFlip
    }
}