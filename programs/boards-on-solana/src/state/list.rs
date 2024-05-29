use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct List {
    pub list_id: u8,
    pub name: String,
    pub bounty_payout_percentage: u8,
}


impl List {

    pub fn new(
        list_id: u8,
        name: String,
        bounty_payout_percentage: u8,
    ) -> Self {

        Self {
            list_id,
            name,
            bounty_payout_percentage,
        }

    }

    pub fn new_default() -> Self {

        Self {
            list_id: 1,
            name: String::from("Todo"),
            bounty_payout_percentage: 0,
        }

    }

}