use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ near_bindgen, AccountId, PanicOnDefault};
use crate::proposal::{Proposal, ProposalContract, ProposalState};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
impl ProposalContract {
    // Cast a vote on a specific proposal
    pub fn vote(&mut self, proposal_id: u64, voter: AccountId, vote_option: u8) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");

        assert!(env::block_timestamp() <= proposal.deadline, "Voting period has ended");
        assert!(proposal.state == ProposalState::Open, "Proposal is not open for voting");
        assert!(!proposal.votes.contains_key(&voter), "Voter has already voted");
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

    // Count votes and return status
    pub fn count_votes(&self, proposal_id: u64) -> ProposalState {
        let proposal = self.proposals.get(&proposal_id).expect("Proposal not found.");
    
        let mut votes_for_option_a = 0;
        let mut votes_for_option_b = 0;
    
        // Iterate through votes to count for each option
        for (_account, &vote) in proposal.votes.iter() {
            if vote == 0 { // Assuming 0 represents Option A
                votes_for_option_a += 1;
            } else if vote == 1 { // Assuming 1 represents Option B
                votes_for_option_b += 1;
            }
        }
    
        // Determine the outcome based on vote counts and minimum votes requirement
        if votes_for_option_a > votes_for_option_b && votes_for_option_a >= proposal.minimum_votes as u64 {
            ProposalState::Passed
        } else if votes_for_option_b > votes_for_option_a && votes_for_option_b >= proposal.minimum_votes as u64 {
            ProposalState::Passed
        } else {
            ProposalState::Rejected
        }
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