use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub name: String,
    pub game: AvailableGames,
    pub score: u64,
}

impl Game {
    pub const SEED_PREFIX: &'static str = "game";

    pub const SPACE: usize = 8      // Discriminator
        + 32                        // Vec (empty)
        + 32                        // Vec (empty)
        + 32;                       // Pubkey

    // To be added
    pub fn new() -> Result<Self> {
        Ok(())
    }
}

// Sample games: will eventually be populated with our first few games
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AvailableGames {
    Game1,
    Game2,
    Game3,
}