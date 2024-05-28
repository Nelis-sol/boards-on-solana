use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_data;
use std::str::FromStr;

use crate::constants::*;
use crate::state::{Card, List};

#[account]
pub struct Board {
    pub seed: u64, // 8 byte
    pub url: String, // 4 + length of string in bytes
    pub members: Vec<(Pubkey, u8)>, // 4 + (32 pubkey + 1 role indicator) * number of members
    pub lists: Vec<List>, // 4 + (1 list index + 4+ list name) * number of lists
    pub cards: Vec<Card>, // 4 + (1 list index + bounty) * number of cards
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
            self.lists.push(List::new_default());

            // start with default card attached to list 1 and 0 bounty
            self.cards.push(Card::new_default());

            // take USDC as default currency
            self.currency = Pubkey::from_str(DEFAULT_BOUNTY_CURRENCY).unwrap();

            self.bump = bump;

            msg!("Board: {} - url: {} - members: {:#?} - lists: {:#?} - cards: {:#?} - currency: {}",
                    self.seed, self.url, self.members, self.lists, self.cards, self.currency);

            Ok(())
    }


    pub fn add_list_to_board(
        &mut self,
        list_name: String,
        bounty_payout_percentage: u8,
    ) -> Result<()> {

        // TODO: come up with a better way for list_id's
        let list_id: u8 = self.lists.len().try_into().unwrap();

        // Create new list item
        let list = List::new(list_id, list_name, bounty_payout_percentage);

        // Add list to board
        self.lists.push(list);

        msg!("Lists: {:#?}", self.lists);

        Ok(())
    }


    pub fn add_card_to_board(
        &mut self,
        list_id: u8,
        bounty: u64,
    ) -> Result<()> {

        // TODO: come up with a better way for card_id's
        let card_id: u8 = self.cards.len().try_into().unwrap();

        // Create new card item
        let card = Card::new(card_id, list_id, bounty);

        // Add card to the board
        self.cards.push(card);

        msg!("Cards: {:#?}", self.cards);

        Ok(())
    }


    pub fn move_card_to_list(
        &mut self,
        card_id: u8,
        list_id: u8,
    ) -> Result<()> {

        let card = self.find_card_by_id(card_id).expect("card id not found in card vector");
        
        card.move_card(list_id);

        msg!("Cards: {:#?}", self.cards);

        Ok(())
    }


    pub fn find_card_by_id(
        &mut self, 
        card_id: u8
    ) -> Option<&mut Card> {

        self.cards.iter_mut().find(|card| card.card_id == card_id)

    }

}

