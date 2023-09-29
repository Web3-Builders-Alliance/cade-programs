use anchor_lang::prelude::*;

pub fn create_profile(ctx: Context<CreateAvatar>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    pub system_program: Program<'info, System>
}