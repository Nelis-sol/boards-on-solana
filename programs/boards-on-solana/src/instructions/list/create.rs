use anchor_lang::prelude::*;
use crate::state::Board;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"board", seed.to_le_bytes().as_ref()],
        realloc = std::mem::size_of::<Board>()+5,
        realloc::payer = payer,
        realloc::zero = false,
        bump,
      )]
    // PDA that contains the board information (including lists, cards)
    pub board: Account<'info, Board>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateList<'info> {

    pub fn add_list_to_board(
        &mut self, 
        list_name: String,
        bounty_payout_percentage: u8,
        bumps: &CreateListBumps
    ) -> Result<()> {

        self.board.add_list_to_board(
            list_name,
            bounty_payout_percentage,
        )
        
    }

}
