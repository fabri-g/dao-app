// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk:: {
    env, 
    near_bindgen,
    AccountId,
    PanicOnDefault,
    Gas,
    ext_contract, 
    NearToken
};

//DAO Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DAO {
    admin: AccountId,
    proposal_contract_id: AccountId,
}

// Cross-Contract Call Needed
#[ext_contract(proposal_contract)]
pub trait ProposalContractInterface {
    fn create_proposal(&mut self, title: String, description: String, deadline: u64, options: Vec<String>, minimum_votes: u8);
    fn update_status(&mut self, proposal_id: u64);
}

// Implement the DAO structure
#[near_bindgen]
impl DAO {
    #[init]
    pub fn new(admin_account_id: AccountId, proposal_contract_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            admin: admin_account_id,
            proposal_contract_id,
        }
    }

    pub fn create_proposal(&mut self, title: String, description: String, deadline: u64, options: Vec<String>, minimum_votes: u8) {
        // Verify the caller is the admin
        assert_eq!(env::predecessor_account_id(), self.admin, "Only the admin can create proposals");

        proposal_contract::ext(self.proposal_contract_id.clone())
            .with_attached_deposit(NearToken::from_near(0))
            .with_static_gas(Gas::from_tgas(20))
            .create_proposal(
                title,
                description,
                deadline,
                options,
                minimum_votes,
            );
    }

    pub fn finalize_proposal(&mut self, proposal_id: u64) {
        // Verify the caller is the admin
        assert_eq!(env::predecessor_account_id(), self.admin, "Only the admin can finalize proposals");

        proposal_contract::ext(self.proposal_contract_id.clone())
            .with_attached_deposit(NearToken::from_near(0))
            .with_static_gas(Gas::from_tgas(5))
            .update_status(proposal_id);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::PublicKey;
    use std::str::FromStr;

    // Use `cargo test -- --nocapture` to view logs
    #[test]
    fn test_create_proposal() {
        let context = get_context("admin".parse().unwrap());
        testing_env!(context);
        let mut contract = DAO::new("admin".parse().unwrap(), "proposal".parse().unwrap());
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
    }

    #[test]
    fn test_finalize_proposal() {
        let context = get_context("admin".parse().unwrap());
        testing_env!(context);
        let mut contract = DAO::new("admin".parse().unwrap(), "proposal".parse().unwrap());
        contract.finalize_proposal(0);
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: "dao".parse().unwrap(),
            signer_account_id: "signer".parse().unwrap(),
            signer_account_pk:  PublicKey::from_str("ed25519:3tH4yM9oYuZFUHX6SxKJEzDiQUDfydBKH4rXXQbVZxjj")
                                .expect("Failed to create public key"),
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: NearToken::from_yoctonear(0),
            account_locked_balance: NearToken::from_yoctonear(0),
            storage_usage: 10u64.pow(6),
            attached_deposit: NearToken::from_yoctonear(0),
            prepaid_gas: Gas::from_tgas(200),
            random_seed: [0u8; 32],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    #[should_panic(expected = "Only the admin can create proposals")]
    fn test_create_proposal_not_admin() {
        let context = get_context("not_admin".parse().unwrap());
        testing_env!(context);
        let mut contract = DAO::new("admin".parse().unwrap(), "proposal".parse().unwrap());
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
    }

    #[test]
    #[should_panic(expected = "Only the admin can finalize proposals")]
    fn test_finalize_proposal_not_admin() {
        let context = get_context("not_admin".parse().unwrap());
        testing_env!(context);
        let mut contract = DAO::new("admin".parse().unwrap(), "proposal".parse().unwrap());
        contract.finalize_proposal(0);
    }

    #[test]
    fn test_init() {
        let context = get_context("admin".parse().unwrap());
        testing_env!(context);
        let contract = DAO::new("admin".parse().unwrap(), "proposal".parse().unwrap());
        assert_eq!(contract.admin, "admin".to_string());
        assert_eq!(contract.proposal_contract_id, "proposal".to_string());
    }
}