use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

pub use state::*;
pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod avatar_program {
    use super::*;

    pub fn create_avatar(ctx: Context<CreateAvatar>) -> Result<()> {
        instructions::create_avatar(ctx);
    }

    pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
        instructions::create_profile(ctx);
    }
}
