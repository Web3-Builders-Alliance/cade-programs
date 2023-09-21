use anchor_lang::prelude::*;

declare_id!("F18piyUM9ccShzRZMG8y6TmQT8YttYB5mPW6WY2LyMkH");

#[program]
pub mod dtt {

    use super::*;
    
    pub fn create_map(
        ctx: Context<CreateMap>,
        name: String,
        board: Vec<Element>,
        budget: u64,
    ) -> Result<()> {
        let map = &mut ctx.accounts.map;
        map.name = name;
        map.board = board.to_vec();
        map.authority = *ctx.accounts.user.key;
        map.budget = budget;
        map.bump = *ctx.bumps.get("map").unwrap();
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let map = &mut ctx.accounts.map;
        game.player = *ctx.accounts.user.key;
        game.status = "created".to_string();
        game.points = 0;
        game.map = *map.to_account_info().key;

        Ok(())
    }

    pub fn deploy_units(ctx: Context<DeployUnits>, deploys: [Vec<String>; 6]) -> Result<()> {
        let game = &mut *ctx.accounts.game;
        let map = &mut ctx.accounts.map;
        let units : [Unit;3] = [
        Unit {
            kind: "soldier".to_string(),
            health: 100,
            dps: 10,
            cost: 10,
        },
        Unit {
            kind: "tank".to_string(),
            health: 200,
            dps: 25,
            cost: 20,
        },
        Unit {
            kind: "plane".to_string(),
            health: 50,
            dps: 75,
            cost: 50,
        },
    ];
        let mut budget = map.budget;
        for deploy in deploys.iter() {
            let mut count = 0;
            for unit_to_deploy in deploy.iter() {
                let unit = units.iter().find(|unit| unit.kind == *unit_to_deploy);
                match unit {
                    Some(unit) => {
                        count += unit.cost as u64;
                    }
                    None => {
                        return Err(ErrorCode::InvalidUnit.into());
                    }
                }
                if count > budget {
                    return Err(ErrorCode::CostExceedsBudget.into());
                }
                budget -= count;
            }
        }
        game.points += budget;
        game.deploys = deploys;
        game.status = "deployed".to_string();
        Ok(())
    }

    pub fn resolve_game(ctx: Context<ResolveGame>) -> Result<()>{
        let game = &mut *ctx.accounts.game;
        let map = &mut ctx.accounts.map;
        let units : [Unit;3] = [
        Unit {
            kind: "soldier".to_string(),
            health: 100,
            dps: 10,
            cost: 10,
        },
        Unit {
            kind: "tank".to_string(),
            health: 200,
            dps: 25,
            cost: 20,
        },
        Unit {
            kind: "plane".to_string(),
            health: 50,
            dps: 75,
            cost: 50,
        },
    ];
        let mut win= false;
        let mut points = game.points;
        for i in 0 as u8 .. 6 as u8 {
            msg!("Line {}", i);
            let elements_in_line = map.board.iter().filter(|element| element.position / 6 == i);
            let attakers_dps = game.deploys[i as usize].iter().fold(0, |acc, unit| acc + units.iter().find(|u| u.kind == *unit).unwrap().dps as u64);
            let attakers_health = game.deploys[i as usize].iter().fold(0, |acc, unit| acc + units.iter().find(|u| u.kind == *unit).unwrap().health as u64);
            let line_damage = elements_in_line.fold(0, |acc, element| acc + ((element.dps*element.health) as u64 / attakers_dps));
            if attakers_health > line_damage {
                win = true;
                points += map.board.iter().filter(|element| element.position / i == 0).fold(0, |acc, element| acc + element.health as u64);
            } else {
                points += attakers_dps * attakers_health / line_damage;
            }
        }
        game.points = if win {points + 100 } else {points};
        game.status = "resolved".to_string();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateMap<'info> {
    #[account(
        init,
        payer = user,
        space = 10000,
        seeds = [name.as_ref()],
        bump
    )]
    pub map: Account<'info, Map>,
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
        space = 1000,
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    /// CHECK: 
    pub user: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveGame<'info> {
    pub map: Account<'info, Map>,
    #[account(
        mut
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeployUnits<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub map: Account<'info, Map>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub map: Pubkey,
    pub deploys: [Vec<String>; 6],
    pub points: u64,
    pub status: String,
}

#[account]
pub struct Map {
    pub name: String,
    pub board: Vec<Element>,
    pub budget: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Element {
    pub kind: String,
    pub health: u8,
    pub dps: u8,
    pub position: u8,
}

#[account]
pub struct Unit {
    pub kind: String,
    pub health: u8,
    pub dps: u8,
    pub cost: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Cost exceeds budget")]
    CostExceedsBudget,
    #[msg("Tryied to Deploy an invalid Unit")]
    InvalidUnit,
}
