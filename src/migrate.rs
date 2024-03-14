use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldContract {
    greeting: String,
    counter: usize,
    encryption_pub_key: String,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        // retrieve the current state from the contract
        let old_state: OldContract = env::state_read().expect("failed");

        // return the new state
        Self {
            ipfs_cid: "".to_string(),
            encryption_pub_key: old_state.encryption_pub_key,
        }
    }
}
