// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::UnorderedMap;

// Modules
mod proposal;
use proposal::ProposalContract;

mod vote;

//DAO Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DAO {
    admin: AccountId,
}

// Implement the DAO structure
#[near_bindgen]
impl DAO {
    #[init]
    pub fn new(admin_account_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            admin: admin_account_id,
        }
    }

    pub fn create_proposal(&mut self, title: String, description: String, deadline: u64, options: Vec<String>) -> u64 {
        // Verify the caller is the admin
        assert_eq!(env::predecessor_account_id(), self.admin, "Only the admin can create proposals");

        // Initialize ProposalContract instance
        let mut proposal_contract = ProposalContract::new();

        // Create proposal using the ProposalContract
        proposal_contract.create_proposal(title, description, deadline, options)
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    #[test]
    fn test_create_proposal() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .finish();
        testing_env!(context);

        let mut dao = DAO::new(accounts(0));
        let proposal_id = dao.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()]);

        assert_eq!(proposal_id, 0);
    }
}
