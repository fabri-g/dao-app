use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ env, near_bindgen, AccountId, ext_contract, Gas};
use near_sdk::json_types::U128;
use crate::proposal::{ProposalContract, ProposalState};

#[ext_contract(ft_contract)]
pub trait FungibleToken {
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

#[near_bindgen]
impl ProposalContract {
    // Cast a vote on a specific proposal
    pub fn vote(&mut self, proposal_id: u64, voter: AccountId, vote_option: u8) {
        ft_contract::ft_balance_of(voter.clone(),)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(5))
                    .process_vote_callback(proposal_id, voter, vote_option)
            );
    }

    #[private]
    pub fn process_vote_callback(&mut self, proposal_id: u64, voter: AccountId, vote_option: u8, #[callback_result] balance: Result<U128, near_sdk::PromiseError>) {
        const MINIMUM_BALANCE_REQUIRED: u128 = 1;
        match balance {
            Ok(balance) => {
                if balance.0 >= MINIMUM_BALANCE_REQUIRED {
                    let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");

                    assert!(env::block_timestamp() <= proposal.deadline, "Voting period has ended");
                    assert!(proposal.state == ProposalState::Open, "Proposal is not open for voting");
                    assert!(!proposal.votes.contains_key(&voter), "Voter has already voted");
                    assert!(proposal.options.get(vote_option as usize).is_some(), "Invalid option");
                    
                    //Register the vote
                    proposal.votes.insert(&voter, &vote_option);
                    self.proposals.insert(&proposal_id, &proposal);

                    env::log_str(&format!("Vote cast by {} for option {}", voter, vote_option));
                } else {
                    env::panic_str("Insufficient balance to vote");
                }
            },
            Err(e) => {
                env::panic_str(&format!("Failed to retrieve balance: {:?}", e));
            }
        }
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
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Use `cargo test -- --nocapture` to view logs
    #[test]
    fn test_vote() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.vote(0, "voter".to_string(), 0);
    }

    #[test]
    fn test_count_votes() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.vote(0, "voter".to_string(), 0);
        assert_eq!(contract.count_votes(0), ProposalState::Open);
    }

    fn get_context(predecessor_account_id: String) -> VMContext {
        VMContext {
            current_account_id: "proposal".to_string(),
            signer_account_id: "signer".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            attached_deposit: 0,
            is_view: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_process_vote_callback() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.process_vote_callback(0, "voter".to_string(), 0, Ok(U128(1)));
    }

    #[test]
    #[should_panic(expected = "Insufficient balance to vote")]
    fn test_process_vote_callback_insufficient_balance() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.process_vote_callback(0, "voter".to_string(), 0, Ok(U128(0)));
    }

    #[test]
    #[should_panic(expected = "Voting period has ended")]
    fn test_process_vote_callback_voting_period_ended() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            0,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.process_vote_callback(0, "voter".to_string(), 0, Ok(U128(1)));
    }

    #[test]
    #[should_panic(expected = "Proposal is not open for voting")]
    fn test_process_vote_callback_proposal_not_open() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.update_status(0);
        contract.process_vote_callback(0, "voter".to_string(), 0, Ok(U128(1)));
    }

    #[test]
    #[should_panic(expected = "Voter has already voted")]
    fn test_process_vote_callback_voter_already_voted() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.vote(0, "voter".to_string(), 0);
        contract.process_vote_callback(0, "voter".to_string(), 0, Ok(U128(1)));
    }

    #[test]
    #[should_panic(expected = "Invalid option")]
    fn test_process_vote_callback_invalid_option() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.process_vote_callback(0, "voter".to_string(), 2, Ok(U128(1)));
    }

    #[test]
    #[should_panic(expected = "Failed to retrieve balance: NotEnoughAllowance")]
    fn test_process_vote_callback_not_enough_allowance() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.process_vote_callback(0, "voter".to_string(), 0, Err(near_sdk::PromiseError::NotEnoughAllowance));
    }

    #[test]
    #[should_panic(expected = "Proposal not found")]
    fn test_process_vote_callback_proposal_not_found() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.process_vote_callback(0, "voter".to_string(), 0, Ok(U128(1)));
    }

    #[test]
    fn test_get_votes() {
        let context = get_context("voter".to_string());
        testing_env!(context);
        let mut contract = ProposalContract::new();
        contract.create_proposal(
            "title".to_string(),
            "description".to_string(),
            1000,
            vec!["option1".to_string(), "option2".to_string()],
            2,
        );
        contract.vote(0, "voter".to_string(), 0);
        assert_eq!(contract.get_votes(0), vec![("option1".to_string(), 1), ("option2".to_string(), 0)]);
    }
}