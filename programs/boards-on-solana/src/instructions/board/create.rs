use anchor_lang::prelude::*;
use crate::state::Board;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateBoard<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer,
        space = std::mem::size_of::<Board>(),
        // ownership is not tied to payer
        seeds = [b"board", seed.to_le_bytes().as_ref()],
        bump,
      )]
    // PDA that contains the board information (including lists, cards)
    pub board: Account<'info, Board>,
    pub system_program: Program<'info, System>,
}


impl<'info> CreateBoard<'info> {

    pub fn new_board(
        &mut self, 
        seed: u64, 
        url: String, 
        bumps: &CreateBoardBumps
    ) -> Result<()> {
        
        // use board method to create new default board
        self.board.new(
            seed,
            url,
            self.payer.key(),
            bumps.board,      
        )
    }

}
