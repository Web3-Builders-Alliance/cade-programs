use anchor_lang::prelude::*;

use crate::state::game::*;

#[account]
pub struct Player {
    pub name: String,
    pub score: u64,
    pub game_ref: Game,
}

impl Player {
    pub fn new(name: String, score: u64, game_ref: Game) -> Result<Self> {
        Ok(Self {
            name,
            score,
            game_ref
        })
    }

    pub fn update_score(&mut self, new_score: u64) -> Result<()> {
        self.score = new_score;

        Ok(())
    }
}