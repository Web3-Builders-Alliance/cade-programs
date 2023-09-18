use anchor_lang::prelude::*;

#[error_code]
pub enum CadeError {
    #[msg("Score is too low to reach the leaderboard")]
    ScoreTooLow,
}