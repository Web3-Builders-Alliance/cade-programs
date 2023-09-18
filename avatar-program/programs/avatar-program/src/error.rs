use anchor_lang::prelude::*;

#[error_code]
pub enum CadeError {
    #[msg("This is a sample error for testing.")]
    SampleError,
}