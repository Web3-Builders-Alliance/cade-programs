use anchor_lang::prelude::*;

declare_id!("4P1gf2p6cji6e23uw4FpKLqXx6ecg12rfBXnk8Mrrdyg");

#[program]
pub mod ddt {
    use super::*;

    pub fn create_map(ctx: Context<CreateMap>, name: String, width: u64, length: u64, budget: u64) -> Result<()> {
        let map = &mut ctx.accounts.map;
        map.name = name;
        map.elements = 0;
        map.width = width;
        map.length = length;
        map.authority = *ctx.accounts.user.key;
        map.budget = budget;
        map.bump = *ctx.bumps.get("map").unwrap();

        Ok(())
    }

    pub fn add_element_to_map(ctx: Context<AddElementToMap>, kind: String, health: u64, dps: u64, gaia: bool, position_x: u64, position_y: u64, index: u64) -> Result<()> {
        let map = &mut ctx.accounts.map;
        let element = &mut ctx.accounts.element;
        element.kind = kind;
        element.health = health;
        element.dps = dps;
        element.gaia = gaia;
        element.position_x = position_x;
        element.position_y = position_y;
        element.bump = *ctx.bumps.get("element").unwrap();
        map.elements += 1;
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let map = &mut ctx.accounts.map;
        game.player = *ctx.accounts.user.key;
        game.started_at = Clock::get()?.slot.checked_add(100).unwrap();
        game.status = "created".to_string();
        game.points = 0;
        game.elements = map.elements;
        game.budget = map.budget;
        game.bump = *ctx.bumps.get("game").unwrap();
        Ok(())
    }

    pub fn add_element_to_game(ctx: Context<AddElementToGame>, kind: String, health: u64, dps: u64, gaia: bool, position_x: u64, position_y: u64, index: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let element = &mut ctx.accounts.element;
        element.kind = kind;
        element.health = health;
        element.dps = dps;
        element.gaia = gaia;
        element.position_x = position_x;
        element.position_y = position_y;
        element.bump = *ctx.bumps.get("element").unwrap();
        game.elements += 1;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateMap<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 8 + 8 + 32 + 1,
        seeds = [name.as_ref()],
        bump
    )]
    pub map: Account<'info, Map>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct AddElementToMap<'info> {
    pub map: Account<'info, Map>,
    #[account(
        init,
        payer = user,
        space = 100,
        seeds = [&index.to_le_bytes()],
        bump
    )]
    pub element: Account<'info, Element>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    pub map: Account<'info, Map>,
    #[account(
        init,
        payer = user,
        space = 100,
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(index: u64)]
pub struct AddElementToGame<'info> {
    pub game: Account<'info, Game>,
    #[account(
        init,
        payer = user,
        space = 100,
        seeds = [&index.to_le_bytes()],
        bump
    )]
    pub element: Account<'info, Element>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub started_at: u64,
    pub elements: u64,
    pub points: u64,
    pub status: String,
    pub budget: u64,
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

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct NewElement {
    pub kind: String,
    pub health: u64,
    pub dps: u64,
    pub gaia: bool, 
    pub position_x: u64,
    pub position_y: u64,
}