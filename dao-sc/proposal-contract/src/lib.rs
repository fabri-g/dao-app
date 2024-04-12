use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault};

mod vote;

// Represent the state of a proposal
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
pub enum ProposalState {
    Open,
    Closed,
    Passed,
    Rejected,
}

//Proposal Structure
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
    token_contract_id: AccountId,
}

// Implement the Proposal Contract
#[near_bindgen]
impl ProposalContract {
    //Iinitializes the contract
    #[init]
    pub fn new(token_contract_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            proposals: UnorderedMap::new(b"p"),
            proposal_count: 0,
            token_contract_id,
        }
    }

    // Create a new proposal
    pub fn create_proposal(&mut self, title: String, description: String, deadline: u64, options_vec: Vec<String>, minimum_votes: u8) -> u64 {
        let proposal_id = self.proposal_count;
        let mut options = Vector::new(b"p");
        for option in options_vec.into_iter() {
            options.push(&option);
        }
        assert!(deadline > env::block_timestamp(), "Deadline must be in the future");
        assert!(options.len() > 1, "At least two options are required");
        let new_proposal = Proposal {
            title: title.clone(),
            description,
            deadline,
            options,
            minimum_votes,
            votes: UnorderedMap::new(format!("v{}", proposal_id).as_bytes()),
            state: ProposalState::Open,
        };
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
        assert!(env::block_timestamp() > proposal.deadline, "Proposal deadline has not passed yet");
        let new_state = self.count_votes(proposal_id);
        proposal.state = new_state.clone();
        self.proposals.insert(&proposal_id, &proposal);
        env::log_str(&format!("Proposal {} status updated to {:?}", proposal_id, new_state));
    }
}

// The rest of this file holds the inline tests for the code above

#[cfg(test)]
mod tests {
    use super::*;
    // use near_sdk::MockedBlockchain;
    use near_sdk::PublicKey;
    use near_sdk::{testing_env, VMContext, NearToken, Gas};
    use std::str::FromStr;

    // Use `cargo test --lib` to run unit tests
    #[test]
    fn test_proposal() {
        let context = get_context("alice".parse().unwrap());
        testing_env!(context);
        let mut contract = ProposalContract::new("token-contract".parse().unwrap());
        let options_vec = vec!["Yes".to_string(), "No".to_string()];
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            options_vec,
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
            current_account_id: "proposal_contract".parse().unwrap(),
            signer_account_id: "proposal_contract".parse().unwrap(),
            signer_account_pk: PublicKey::from_str("ed25519:3tH4yM9oYuZFUHX6SxKJEzDiQUDfydBKH4rXXQbVZxjj")
                                .expect("Failed to create public key"),
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 1_600_000_000_000,
            account_balance: NearToken::from_yoctonear(5),
            account_locked_balance: NearToken::from_yoctonear(0),
            storage_usage: 10u64.pow(6),
            attached_deposit: NearToken::from_yoctonear(0),
            prepaid_gas: Gas::from_tgas(200),
            random_seed: [0u8; 32],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    /* #[test]
    fn test_update_status() {
        let context = get_context("alice".parse().unwrap());
        testing_env!(context);
        let mut contract = ProposalContract::new("token-contract".parse().unwrap());
        let options_vec = vec!["Yes".to_string(), "No".to_string()];
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            options_vec,
            1,
        );
        contract.vote(proposal_id, "alice".parse().unwrap(), 0);
        contract.update_status(proposal_id);
        let proposal = contract.get_proposal(proposal_id).expect("Proposal not found");
        assert_eq!(proposal.state, ProposalState::Passed);
    } */

    #[test]
    fn test_list_proposals() {
        let context = get_context("alice".parse().unwrap());
        testing_env!(context);
        let mut contract = ProposalContract::new("token-contract".parse().unwrap());
        let options_vec = vec!["Yes".to_string(), "No".to_string()];
        let proposal_id = contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp() + 1000,
            options_vec,
            1,
        );
        let proposals = contract.list_proposals();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].0, proposal_id);
    }

    #[test]
    #[should_panic(expected = "Proposal not found")]
    fn test_update_status_invalid_proposal() {
        let context = get_context("alice".parse().unwrap());
        testing_env!(context);
        let mut contract = ProposalContract::new("token-contract".parse().unwrap());
        contract.update_status(0);
    }

    #[test]
    #[should_panic(expected = "Deadline must be in the future")]
    fn test_create_proposal_invalid_deadline() {
        let context = get_context("alice".parse().unwrap());
        testing_env!(context);
        let options_vec = vec!["Yes".to_string(), "No".to_string()];
        let mut contract = ProposalContract::new("token-contract".parse().unwrap());
        contract.create_proposal(
            "Test Proposal".to_string(),
            "This is a test proposal".to_string(),
            env::block_timestamp(),
            options_vec,
            1,
        );
    }
}