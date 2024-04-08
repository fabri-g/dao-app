use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::ext_contract;
use near_sdk::collections::{UnorderedMap, Vector};

// Represent the state of a proposal
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub enum ProposalState {
    Open,
    Closed,
    Passed,
    Rejected,
}

//Proposal Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
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
        assert!(options.len() > 1, "At least two options are required")
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
    pub fn update_status(&mut self, proposal_id: u64, new_state: ProposalState) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        proposal.state = new_state;
        self.proposals.insert(&proposal_id, &proposal);
        env::log_str(&format!("Proposal {} status updated to {:?}", proposal_id, new_state));
    }
}

// The rest of this file holds the inline tests for the code above

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
            .build();
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Option A".to_string(), "Option B".to_string()]),
            1,
        );
        let proposal = contract.get_proposal(proposal_id).expect("Proposal not found");
        assert_eq!(proposal.title, "Test Proposal".to_string());
        assert_eq!(proposal.options.len(), 2);
    }

    #[test]
    fn test_list_proposals() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "Test Proposal 1".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Option A".to_string(), "Option B".to_string()]),
            1,
        );
        contract.create_proposal(
            "Test Proposal 2".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Option A".to_string(), "Option B".to_string()]),
            1,
        );
        let proposals = contract.list_proposals();
        assert_eq!(proposals.len(), 2);
    }

    #[test]
    fn test_update_status() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            Vector::from(vec!["Option A".to_string(), "Option B".to_string()]),
            1,
        );
        contract.update_status(proposal_id, ProposalState::Closed);
        let proposal = contract.get_proposal(proposal_id).expect("Proposal not found");
        assert_eq!(proposal.state, ProposalState::Closed);
    }
}