// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    greeting: String,
    counter: usize,
    encryption_pub_key: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
            counter: 0,
            encryption_pub_key: "".to_string(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        return self.greeting.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log_str(&format!("Saving greeting: {greeting}"));
        self.greeting = greeting;
    }

    pub fn get_encryption_pub_key(&self) -> String {
        self.encryption_pub_key.clone()
    }

    pub fn set_encryption_pub_key(&mut self, key: String) {
        self.encryption_pub_key = key;
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }

    pub fn increment_counter(&mut self) {
        log_str(&format!("Incrementing: {}", self.counter));
        self.counter += 1;
    }

    pub fn multiply_counter(&mut self, val: usize) {
        log_str(&format!("Multiplying: {} with {}", self.counter, val));
        self.counter *= val;
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
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello".to_string());
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy".to_string());
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
