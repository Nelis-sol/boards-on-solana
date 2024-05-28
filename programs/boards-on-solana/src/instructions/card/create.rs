use anchor_lang::prelude::*;
use crate::state::Board;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateCard<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"board", seed.to_le_bytes().as_ref()],
        realloc = std::mem::size_of::<Board>()+7,
        realloc::payer = payer,
        realloc::zero = false,
        bump,
      )]
    // PDA that contains the board information (including lists, cards)
    pub board: Account<'info, Board>,
    pub system_program: Program<'info, System>,
}


impl<'info> CreateCard<'info> {

    pub fn add_card_to_board(
        &mut self, 
        list_index: u8,
        bounty: u64,
        bumps: &CreateCardBumps
    ) -> Result<()> {

        self.board.add_card_to_board(
            list_index, 
            bounty
        )

    }

}