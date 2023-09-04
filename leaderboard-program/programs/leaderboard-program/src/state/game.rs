use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player: Pubkey,
    pub game_id: AvailableGames,
    pub game_program: Pubkey,
    pub score: u64,
    pub timestamp: u64,
}

impl Game {
    pub const SEED_PREFIX: &'static str = "game";

    pub const SPACE: usize = 8      // Discriminator
        + 32                        // Vec (empty)
        + 32                        // Vec (empty)
        + 4                         // Enum (Singleton)
        + 32;                       // Pubkey

    // Creates a new game instance
    pub fn new(
        player: Pubkey, 
        game_id: AvailableGames,
        game_program: Pubkey,
        score: u64, 
    ) -> Result<Self> {
        let current_time = Clock::get()?.unix_timestamp as u64;

        Ok(Self {
            player,
            game_id,
            game_program,
            score,
            current_time
        })
    }
}

// Sample games: will eventually be populated with our first few games
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AvailableGames {
    CoinFlip,
    CadeWars,
    CadeInvaders,
}