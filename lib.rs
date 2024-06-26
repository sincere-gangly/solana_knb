use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWxTWy6WbXDQpkNGRWEyhi2AC1yq");

#[program]
pub mod rock_paper_scissors {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let game = &mut ctx.accounts.game;
        game.player1 = ctx.accounts.player1.key();
        game.player2 = ctx.accounts.player2.key();
        game.player1_move = None;
        game.player2_move = None;
        Ok(())
    }

    pub fn make_move(ctx: Context<MakeMove>, player_move: u8) -> ProgramResult {
        let game = &mut ctx.accounts.game;
        if ctx.accounts.player.key() == game.player1 {
            game.player1_move = Some(player_move);
        } else if ctx.accounts.player.key() == game.player2 {
            game.player2_move = Some(player_move);
        } else {
            return Err(ErrorCode::InvalidPlayer.into());
        }

        if let (Some(p1_move), Some(p2_move)) = (game.player1_move, game.player2_move) {
            let result = determine_winner(p1_move, p2_move);
            game.result = result;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = player1, space = 8 + 32 + 32 + 1 + 1 + 1)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player1: Signer<'info>,
    #[account(mut)]
    pub player2: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

#[account]
pub struct Game {
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub player1_move: Option<u8>,
    pub player2_move: Option<u8>,
    pub result: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid player.")]
    InvalidPlayer,
}

fn determine_winner(player1_move: u8, player2_move: u8) -> u8 {
    match (player1_move, player2_move) {
        (0, 2) | (1, 0) | (2, 1) => 1, // Player 1 wins
        (2, 0) | (0, 1) | (1, 2) => 2, // Player 2 wins
        _ => 0, // Tie
    }
}
