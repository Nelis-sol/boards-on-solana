use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Card {
    pub card_id: u8,
    pub list_id: u8,
    pub bounty: u64,
    // last_update time
}


impl Card {

    pub fn new(
        card_id: u8,
        list_id: u8,
        bounty: u64,
    ) -> Self {

        Self {
            card_id,
            list_id,
            bounty,
        }
    }

    pub fn new_default() -> Self {

        Self {
            card_id: 1,
            list_id: 1,
            bounty: 0,
        }
    }

    pub fn move_card(
        &mut self,
        list_id: u8,
    ) -> Self {

        Self {
            card_id: self.card_id,
            list_id,
            bounty: self.bounty,
        }

    }

}