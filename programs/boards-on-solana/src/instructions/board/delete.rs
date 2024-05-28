use anchor_lang::prelude::*;
use crate::state::Board;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DeleteBoard<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = board.members[0].0 == payer.key(),
        seeds = [b"board", seed.to_le_bytes().as_ref()],
        close = payer,
        bump,
      )]
    // PDA that contains the board information (including lists, cards)
    pub board: Account<'info, Board>,
    pub system_program: Program<'info, System>,
}

