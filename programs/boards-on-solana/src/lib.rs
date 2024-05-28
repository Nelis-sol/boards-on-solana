use anchor_lang::prelude::*;

mod constants;
mod instructions;
use instructions::*;
mod state;


declare_id!("7Yq2LTPkk2ZidFkZ8gX9Nv56nGUQC7TmkHYk59pXAS7q");

#[program]
pub mod boards_on_solana {

    use super::*;

    pub fn new_board(ctx: Context<CreateBoard>, seed: u64, url: String) -> Result<()> {
        ctx.accounts.new_board(seed, url, &ctx.bumps)
    }

    pub fn add_list(ctx: Context<CreateList>, _seed: u64, list_name: String, bounty_payout_percentage: u8) -> Result<()> {
        ctx.accounts.add_list_to_board(list_name, bounty_payout_percentage, &ctx.bumps)
    }

    pub fn add_card(ctx: Context<CreateCard>, _seed: u64, list_index: u8, bounty: u64) -> Result<()> {
        ctx.accounts.add_card_to_board(list_index, bounty, &ctx.bumps)
    }

    pub fn delete_board(ctx: Context<DeleteBoard>, _seed: u64) -> Result<()> {
        Ok(())
    }

    pub fn move_card_to_list(ctx: Context<MoveCardToList>, _seed: u64, card_id: u8, list_id: u8) -> Result<()> {
        ctx.accounts.move_card_to_list(card_id, list_id, &ctx.bumps)
    }

}

