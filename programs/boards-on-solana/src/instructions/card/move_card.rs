use anchor_lang::prelude::*;
use crate::state::Board;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct MoveCardToList<'info> {
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


impl<'info> MoveCardToList<'info> {

    pub fn move_card_to_list(
        &mut self,
        card_id: u8,
        list_id: u8,
        bumps: &MoveCardToListBumps
    ) -> Result<()> {

        self.board.move_card_to_list(
            card_id,
            list_id
        )

    }

}