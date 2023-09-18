use anchor_lang::prelude::*;

declare_id!("4P1gf2p6cji6e23uw4FpKLqXx6ecg12rfBXnk8Mrrdyg");

#[program]
pub mod ddt {
    use super::*;

    pub fn create_map(ctx: Context<CreateMap>, name: String, board: [Element; 60], budget: u64) -> Result<()> {
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
        game.started_at = Clock::get()?.slot.checked_add(100).unwrap();
        game.status = "created".to_string();
        game.points = 0;
        for i in 0..60 {
            game.map[i] = Element {
                kind: map.board[i].kind.to_string(),
                health: map.board[i].health,
                dps: map.board[i].dps,
                gaia: true,
            };
        }

        game.budget = map.budget;
        game.bump = *ctx.bumps.get("game").unwrap();
        Ok(())
    }

    pub fn add_element_to_game(ctx: Context<PlayGame>, deploys: Vec<Deploy>) -> Result<()> {
        let game = &mut *ctx.accounts.game;
        let units= [
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
            }
        ];
        let mut board = game.map.iter().map(|e| e.clone()).collect::<Vec<Element>>();
        let mut budget = game.budget;
        let mut points = game.points;
        for deploy in deploys {
            let unit = units.iter().find(|u| u.kind == deploy.kind).unwrap();
            if budget >= unit.cost {
                board[deploy.position as usize] = Element {
                    kind: unit.kind.to_string(),
                    health: unit.health,
                    dps: unit.dps,
                    gaia: false,
                };
                budget -= unit.cost;
            }else{
                return Err(ErrorCode::CostExceedsBudget.into());
            }
        }

        for i in 0..6 {
            let mut j = 9;
            while j > 0 {
                if board[i*10+j].gaia == false {
                    board[i*10+j-1].health -= board[i*10+j].dps;
                    board[i*10+j].health -= board[i*10+j-1].dps;
                    points += board[i*10+j].dps;
                    if board[i*10+j-1].health <= 0 {
                        board[i*10+j-1] = Element {
                            kind: "".to_string(),
                            health: 0,
                            dps: 0,
                            gaia: true,
                        };
                    }
                }
                j -= 1;
            }
        }
        if board[0].health <= 0 {
            points += 100;
            game.status = "win".to_string();
        } else {
            game.status = "lose".to_string();
        }
        game.budget = budget;
        game.points = points;

        for i in 0..60 {
            game.map[i] = Element {
                kind: board[i].kind.to_string(),
                health: board[i].health,
                dps: board[i].dps,
                gaia: board[i].gaia,
            };
        }

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateMap<'info> {
    #[account(
        init,
        payer = user,
        space = 1000,
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
        space = 100,
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlayGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Game {
    pub player: Pubkey,
    pub started_at: u64,
    pub map: Vec<Element>,
    pub points: u64,
    pub status: String,
    pub budget: u64,
    pub bump: u8,
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
    pub health: u64,
    pub dps: u64,
    pub gaia: bool,
}

#[account]
pub struct Unit {
    pub kind: String,
    pub health: u64,
    pub dps: u64,
    pub cost: u64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Deploy {
    pub kind: String,
    pub position: u8,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Cost exceeds budget")]
    CostExceedsBudget,
}
