use anchor_lang::prelude::*;

pub fn create_avatar(ctx: Context<CreateAvatar>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateAvatar<'info> {
    pub system_program: Program<'info, System>
}