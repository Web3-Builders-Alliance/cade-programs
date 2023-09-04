use anchor_lang::prelude::*;

use crate::state::{game::*, player::*};

#[account]
pub struct Leaderboard {
    pub game_id: AvailableGames,
    pub timestamp: u64,
    pub players: Vec<Player>,
    pub bump: u8,
}

impl Leaderboard {
    pub const SEED_PREFIX: &'static str = "leaderboard";

    pub const SPACE: usize = 8 // Discriminator
    + 4                        // Enum
    + 4                        // u64
    + 32;                      // Vec<Player>

    // Creates a new leaderboard once a new game is played
    // for the first time
    pub fn new_with_game(game_id: AvailableGames, player: Player, bump: u8) -> Result<Self> {
        let timestamp = Clock::get()?.unix_timestamp as u64;
        let players = vec![player];

        Ok(Self {   
            game_id,
            timestamp,
            players,
            bump
        })
    }

    // Adds a score to the leaderboard
    pub fn add_score(&mut self, player: Player) -> Result<()> {
        // *** Need a max length of 10 or else "players" will grow too large
        // 1. Check if "players" has hit max length
        // 2. Loop through vec and return position that player.score is more than
        // 3. If it's less than all of the current items, break function
        // 4. If a position is returned, push to vec at the returned position
        // *** The "players" field could also be an array of length 10 with sample scores
        // *** However, I will avoid this for now, as we don't have enough information about the games
        self.players.push(player);

        Ok(())
    }
}