mod migrate;

// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    ipfs_cid: String,
    encryption_pub_key: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            ipfs_cid: "".to_string(),
            encryption_pub_key: "".to_string(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_ipfs_cid(&self) -> &str {
        &self.ipfs_cid
    }

    pub fn set_ipfs_cid(&mut self, ipfs_cid: String) {
        log_str(&format!("Saving ipfs_cid: {ipfs_cid}"));
        self.ipfs_cid = ipfs_cid;
    }

    pub fn get_encryption_pub_key(&self) -> String {
        self.encryption_pub_key.clone()
    }

    pub fn set_encryption_pub_key(&mut self, key: String) {
        self.encryption_pub_key = key;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_ipfs_cid() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_ipfs_cid(), "".to_string());
    }

    #[test]
    fn set_then_get_default_ipfs_cid() {
        let mut contract = Contract::default();
        contract.set_ipfs_cid("howdy".to_string());
        assert_eq!(contract.get_ipfs_cid(), "howdy".to_string());
    }

    #[test]
    fn get_default_encryption_pub_key() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_encryption_pub_key(), "".to_string());
    }

    #[test]
    fn set_then_get_encryption_pub_key() {
        let mut contract = Contract::default();
        contract.set_encryption_pub_key("1239041zasdwaz".to_string());
        assert_eq!(
            contract.get_encryption_pub_key(),
            "1239041zasdwaz".to_string()
        );
    }
}
