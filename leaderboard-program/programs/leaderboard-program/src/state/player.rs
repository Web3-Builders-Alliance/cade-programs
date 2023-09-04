use anchor_lang::prelude::*;

use crate::state::game::*;

#[account]
pub struct Player {
    pub name: String,
    pub score: u64,
    pub game_ref: Game,
}

impl Player {
    // To be added
    pub fn new(name: String, score: u64, game_ref: Game) -> Result<Self> {
        Ok(Self{
            name,
            score,
            game_ref
        })
    }

    // To be added
    pub fn update_score() -> Result<()> {
        Ok(())
    }
}