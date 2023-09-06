use anchor_lang::prelude::*;

declare_id!("4P1gf2p6cji6e23uw4FpKLqXx6ecg12rfBXnk8Mrrdyg");

#[program]
pub mod ddt {
    use super::*;

    pub fn create_map(ctx: Context<CreateMap>, name: String, elements: u64,width: u64, length: u64,) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMap {
    #[account(init, payer = user, space = 8 + 8 + 8 + 8 + 32 + 1)]
    pub map: Account<Map>,
    #[account(mut)]
    pub user: Signer,
    pub system_program: Program<System>,
}


#[account]
pub struct Game {
    pub player: Pubkey,
    pub started_at: u64,
    pub elements: u64,
    pub points: u64,
    pub status: String,
    pub bump: u8,
}

#[account]
pub struct Map {
    pub name: String,
    pub elements: u64,
    pub width: u64,
    pub length: u64,
    pub authority: Pubkey,
    pub budget: u64,
    pub bump: u8,
}

#[account]
pub struct Element {
    pub kind: String,
    pub health: u64,
    pub dps: u64,
    pub gaia: bool, 
    pub position_x: u64,
    pub position_y: u64,
    pub bump: u8,
}

#[account]
pub struct ElementKind {
    pub name: String,
    pub health: u64,
    pub dps: u64,
    pub price: u64,
    pub bump: u8,
}

pub struct NewElement {
    pub kind: String,
    pub health: u64,
    pub dps: u64,
    pub gaia: bool, 
    pub position_x: u64,
    pub position_y: u64,
}