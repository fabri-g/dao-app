use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault};
use serde::{Deserialize, Serialize};
use crate::vote;

// Represent the state of a proposal
#[derive(BorshDeserialize, BorshSerialize)]
pub enum ProposalState {
    Open,
    Closed,
    Passed,
    Rejected,
}

//Proposal Structure
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize)]
pub struct Proposal {
    title: String, 
    description: String,
    deadline: u64,
    options: Vector<String>,
    minimum_votes: u8,
    votes: UnorderedMap<AccountId, u8>,
    state: ProposalState,
}

//Proposal Contract Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ProposalContract {
    proposals: UnorderedMap<u64, Proposal>,
    proposal_count: u64,
}



// Implement the Proposal Contract
#[near_bindgen]
impl ProposalContract {
    //Iinitializes the contract
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            proposals: UnorderedMap::new(b"p"),
            proposal_count: 0,
        }
    }

    // Create a new proposal
    pub fn create_proposal(&mut self, title: String, description: String, deadline: u64, options: Vector<String>, minimum_votes: u8) -> u64 {
        let proposal_id = self.proposal_count;
        let new_proposal = Proposal {
            title,
            description,
            deadline,
            options: Vector::from(options),
            minimum_votes,
            votes: UnorderedMap::new(format!("v{}", proposal_id).as_bytes()),
            state: ProposalState::Open,
        };
        assert!(deadline > env::block_timestamp(), "Deadline must be in the future");
        assert!(options.len() > 1, "At least two options are required");
        self.proposals.insert(&proposal_id, &new_proposal);
        self.proposal_count += 1;
        env::log_str(&format!("Proposal {} created: '{}'", proposal_id, title));
        proposal_id
    }

    // Get a proposal
    pub fn get_proposal(&self, proposal_id: u64) -> Option<Proposal> {
        self.proposals.get(&proposal_id)
    }

    // List all proposals
    pub fn list_proposals(&self) -> Vec<(u64, Proposal)> {
        self.proposals.iter().collect()
    }

    // Update proposal status
    pub fn update_status(&mut self, proposal_id: u64) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        let new_state = self.count_votes(proposal_id);
        proposal.state = new_state;
        self.proposals.insert(&proposal_id, &proposal);
        env::log_str(&format!("Proposal {} status updated to {:?}", proposal_id, new_state));
    }
}

// The rest of this file holds the inline tests for the code above

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Use `cargo test --lib` to run unit tests
    #[test]
    fn test_proposal() {
        let context = get_context("alice");
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Yes".to_string(), "No".to_string()]),
            1,
        );
        let proposal = contract.get_proposal(proposal_id).expect("Proposal not found");
        assert_eq!(proposal.title, "Test Proposal".to_string());
        assert_eq!(proposal.state, ProposalState::Open);
        assert_eq!(proposal.options.len(), 2);
        assert_eq!(proposal.options.get(0).unwrap(), "Yes".to_string());
        assert_eq!(proposal.options.get(1).unwrap(), "No".to_string());
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: "proposal_contract".to_string(),
            signer_account_id: "proposal_contract".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 1_600_000_000_000,
            account_balance: 10u128.pow(26),
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(15),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_vote() {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Yes".to_string(), "No".to_string()]),
            1,
        );
        contract.vote(proposal_id, 0);
        let proposal = contract.get_proposal(proposal_id).expect("Proposal not found");
        assert_eq!(proposal.votes.get("alice".as_bytes()).unwrap(), 0);
    }

    #[test]
    fn test_update_status() {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Yes".to_string(), "No".to_string()]),
            1,
        );
        contract.vote(proposal_id, 0);
        contract.update_status(proposal_id);
        let proposal = contract.get_proposal(proposal_id).expect("Proposal not found");
        assert_eq!(proposal.state, ProposalState::Passed);
    }

    #[test]
    fn test_list_proposals() {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Yes".to_string(), "No".to_string()]),
            1,
        );
        let proposals = contract.list_proposals();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].0, proposal_id);
    }

    #[test]
    #[should_panic(expected = "Proposal not found")]
    fn test_vote_invalid_proposal() {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.vote(0, 0);
    }

    #[test]
    #[should_panic(expected = "Proposal not found")]
    fn test_update_status_invalid_proposal() {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.update_status(0);
    }

    #[test]
    #[should_panic(expected = "Deadline must be in the future")]
    fn test_create_proposal_invalid_deadline() {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp(),
            Vector::from(vec!["Yes".to_string(), "No".to_string()]),
            1,
        );
    }

    #[test]
    test_list_proposals {
        let context = get_context("alice".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Yes".to_string(), "No".to_string()]),
            1,
        );
        let proposals = contract.list_proposals();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].0, proposal_id);
    }
}