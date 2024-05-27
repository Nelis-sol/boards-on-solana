use anchor_lang::prelude::*;
use std::str::FromStr;

use crate::constants::*;

#[account]
pub struct Board {
    pub seed: u64, // 8 byte
    pub url: String, // 4 + length of string in bytes
    pub members: Vec<(Pubkey, u8)>, // 4 + (32 pubkey + 1 role indicator) * number of members
    pub lists: Vec<(u8, String)>, // 4 + (1 list index + 4+ list name) * number of lists
    pub cards: Vec<(u8, u64)>, // 4 + (1 list index + bounty) * number of cards
    pub currency: Pubkey, // 32 bytes
    pub bump: u8, // 1 byte
}

impl Space for Board {
    const INIT_SPACE: usize = 8 + 8 + 4 + 50 + 4 + (32 + 1) * 1 + 4 + (1 + 4 + 10) * 1 + 4 + (1 + 8) * 1 + 32 + 1;
    // total: 164 bytes
}


impl Board {

    pub fn new(
        &mut self,
        seed: u64,
        url: String,
        authority: Pubkey,
        bump: u8,
        ) -> Result<()> {

            self.seed = seed;

            if url.len() <= URL_MAX_SIZE {
                self.url = url
            } else {
                // return error
            }

            // authority is the first member with rol id 1 (= admin)
            self.members.push((authority, 1));

            // start with default list with id 1 and is called "Todo"
            self.lists.push((1, String::from("Todo")));

            // start with default card attached to list 1 and 0 bounty
            self.cards.push((1, 0));

            // take USDC as default currency
            self.currency = Pubkey::from_str(DEFAULT_BOUNTY_CURRENCY).unwrap();

            self.bump = bump;

            Ok(())
    }


    pub fn add_list_to_board(
        &mut self,
        list_name: String,
    ) -> Result<()> {

        let index: u8 = self.lists.len().try_into().unwrap();
        self.lists.push((index, list_name));

        msg!("Lists: {:?}", self.lists);

        Ok(())
    }


    pub fn add_card_to_board(
        &mut self,
        list_index: u8,
        bounty: u64,
    ) -> Result<()> {

        self.cards.push((list_index, bounty));

        msg!("Cards: {:?}", self.cards);

        Ok(())
    }

}

