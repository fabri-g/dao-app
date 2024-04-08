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
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    #[test]
    fn test_dao() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .finish();
        testing_env!(context);

        let mut contract = DAO::new(accounts(0));
        let proposal_id = contract.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()], 1);
        contract.finalize_proposal(proposal_id);
    }

    #[test]
    #[should_panic(expected = "Only the admin can create proposals")]
    fn test_dao_create_proposal_fail() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .finish();
        testing_env!(context);

        let mut contract = DAO::new(accounts(1));
        contract.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()], 1);
    }

    #[test]
    fn test_dao_create_proposal() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .finish();
        testing_env!(context);

        let mut contract = DAO::new(accounts(0));
        let proposal_id = contract.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()], 1);
        assert_eq!(proposal_id, 0);
    }

    #[test]
    fn test_dao_finalize_proposal() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .finish();
        testing_env!(context);

        let mut contract = DAO::new(accounts(0));
        let proposal_id = contract.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()], 1);
        contract.finalize_proposal(proposal_id);
    }

    #[test]
    #[should_panic(expected = "Only the admin can create proposals")]
    fn test_dao_finalize_proposal_fail() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .finish();
        testing_env!(context);

        let mut contract = DAO::new(accounts(0));
        let proposal_id = contract.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()], 1);
        contract.finalize_proposal(proposal_id);
    }
}
