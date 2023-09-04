use anchor_lang::prelude::*;

#[account]
pub struct Player {
    pub name: String,
    pub score: String,
    pub game_ref: Game,
}

impl Player {
    // To be added
    pub fn new() -> Result<Self> {
        Ok(Self{})
    }

    // To be added
    pub fn update_score() -> Result<()> {
        Ok(())
    }
}