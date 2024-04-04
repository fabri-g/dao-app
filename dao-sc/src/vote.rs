use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use crate::proposal::{Proposal, ProposalContract};

#[near_bindgen]
impl ProposalContract {
    // Cast a vote on a specific proposal
    pub fn vote(&mut self, proposal_id: u64, voter: AccountId, vote_option: u8) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");

        // Check for voting eligibility, deadline, and option validity
        assert!(env::block_timestamp() < proposal.deadline, "Voting is closed");
        assert!(proposal.options.get(vote_option as usize).is_some(), "Invalid option");
        
        //Register the vote
        proposal.votes.insert(&voter, &vote_option);
        self.proposals.insert(&proposal_id, &proposal);

        env::log_str(&format!("Vote cast by {} for option {}", voter, vote_option));
    }

    // Get the votes for a specific proposal
    pub fn get_votes(&self, proposal_id: u64) -> Vec<(String, u64)> {
        let proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        let mut votes: Vec<(String, u64)> = proposal.options.iter().map(|option| (option, 0)).collect();
        for (_voter, &vote_option) in proposal.votes.iter() {
            let option_index = vote_option as usize;
            if let Some((_option, count)) = votes.get_mut(option_index) {
                *count += 1;
            }
        }
        votes
    }
}

// The rest of this file holds the inline tests for the code above

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    #[test]
    fn test_vote() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .finish();
        testing_env!(context);

        let mut contract = ProposalContract::new();
        contract.create_proposal("Test Proposal".to_string(), "This is a test proposal".to_string(), 1000, vec!["Option 1".to_string(), "Option 2".to_string()]);
        contract.vote(0, accounts(1), 0);
        contract.vote(0, accounts(2), 1);

        let votes = contract.get_votes(0);
        assert_eq!(votes, vec![("Option 1".to_string(), 1), ("Option 2".to_string(), 1)]);
    }
}