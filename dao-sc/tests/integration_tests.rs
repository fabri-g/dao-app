use near_sdk::AccountId;
use near_workspaces::{Worker, Account, Contract, types::NearToken};
use serde_json::json;

#[tokio::test]
async fn test_create_proposal_and_vote() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let root = worker.root_account();

    // Compile and deploy each contract
    let dao_contract_wasm = near_workspaces::compile_project("./dao-contract").await?;
    let proposal_contract_wasm = near_workspaces::compile_project("./proposal-contract").await?;
    let token_contract_wasm = near_workspaces::compile_project("./token-contract").await?;

    let dao_contract = root.deploy(&dao_contract_wasm).await?;
    let proposal_contract = root.deploy(&proposal_contract_wasm).await?;
    let token_contract = root.deploy(&token_contract_wasm).await?;

    // Initialize contracts 
    dao_contract.call("new")
        .args_json(json!({
            "admin_account_id": root.id(),
            "proposal_contract_id": proposal_contract.id()
        }))
        .transact()
        .await?;

    // Create a proposal using the DAO contract
    let create_proposal_outcome = dao_contract
        .call("create_proposal")
        .args_json(serde_json::json!({
            "title": "New Proposal",
            "description": "Description here",
            "deadline": 1000000, // Example timestamp
            "options": ["Yes", "No"],
            "minimum_votes": 1
        }))?
        .transact().await?;
    
    let proposal_id: u64 = create_proposal_outcome.json()?;

    // Issue tokens to a voter account using the token contract
    token_contract.call("mint")
        .args_json(serde_json::json!({
            "account_id": "voter.testnet",
            "amount": "1000"
        }))?
        .transact().await?;
    
    // Cast a vote on the proposal
    proposal_contract
        .call("vote")
        .args_json(serde_json::json!({
            "proposal_id": proposal_id,
            "voter": "voter.testnet",
            "vote_option": 0 // Yes
        }))?
        .transact().await?;
    
    // Verify the vote was cast
    let votes: Vec<(String, u64)> = proposal_contract.call("get_votes")
        .args_json(serde_json::json!({
            "proposal_id": proposal_id
        }))
        .view()
        .await?
        .json()?;

        let yes_votes = votes.iter().find(|(option, _)| option == "Yes").map(|(_, count)| *count).unwrap_or(0);
        assert_eq!(yes_votes, 1, "The 'Yes' option should have 1 vote.");

    Ok(())
}
