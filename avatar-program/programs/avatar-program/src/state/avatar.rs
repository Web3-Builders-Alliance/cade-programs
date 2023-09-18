use anchor_lang::prelude::*;

#[account]
pub struct Avatar {
    pub nonce: u8,
}