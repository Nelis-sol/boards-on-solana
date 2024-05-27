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

    pub fn add_list(ctx: Context<UpdateBoard>, _seed: u64, list_name: String) -> Result<()> {
        ctx.accounts.add_list_to_board(list_name, &ctx.bumps)
    }

    pub fn add_card(ctx: Context<UpdateBoard>, _seed: u64, list_index: u8, bounty: u64) -> Result<()> {
        ctx.accounts.add_card_to_board(list_index, bounty, &ctx.bumps)
    }

}

