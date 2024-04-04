use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::ext_contract;
use near_sdk::collections::{UnorderedMap, Vector};

//Proposal Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Proposal {
    title: String, 
    description: String,
    deadline: u64,
    options: Vector<String>,
    votes: UnorderedMap<AccountId, u8>,
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
    pub fn create_proposal(&mut self, title: String, description: String, deadline: u64, options: Vector<String>) {
        let proposal_id = self.proposal_count;
        let new_proposal = Proposal {
            title,
            description,
            deadline,
            options: Vector::from(options),
            votes: UnorderedMap::new(format!("v{}", proposal_id).as_bytes()),
        };
        self.proposals.insert(&proposal_id, &new_proposal);
        self.proposal_count += 1;
        env::log_str(&format!("Proposal {} created", proposal_id));
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
}

#[ext_contract(ext_proposal)]
pub trait ExtProposalContract {
    fn get_proposal(&self, proposal_id: u64) -> Proposal;
}

// The rest of this file holds the inline tests for the code above

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, AccountId};

    // Helper function to set up the environment
    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(AccountId::new_unchecked("dao.testnet".to_string()));
        if is_view {
            builder.is_view(is_view);
        }
        builder
    }

    #[test]
    fn create_and_retrieve_proposal() {
        let context = get_context(false);
        testing_env!(context.build());

        let mut contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Proposal Title".to_string(),
            "Proposal Description".to_string(),
            1_000_000_000, 
            vec![
                "Option A".to_string(),
                "Option B".to_string()
            ]
        );

        let proposal = contract.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.title, "Proposal Title".to_string());
        assert_eq!(proposal.description, "Proposal Description".to_string());
        assert_eq!(proposal.deadline, 1_000_000_000);
        assert_eq!(proposal.options.len(), 2);
        assert_eq!(proposal.votes.len(), 0);
    }

    #[test]
    fn list_proposals() {
        let context = get_context(true);
        testing_env!(context.build());

        let contract = ProposalContract::new();
        let proposal_id = contract.create_proposal(
            "Proposal Title".to_string(),
            "Proposal Description".to_string(),
            1_000_000_000, 
            vec![
                "Option A".to_string(),
                "Option B".to_string()
            ]
        );

        let proposals = contract.list_proposals();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].0, proposal_id);
        assert_eq!(proposals[0].1.title, "Proposal Title".to_string());
    }
}
