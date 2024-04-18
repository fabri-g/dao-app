use near_workspaces::{types::NearToken};
use serde_json::json;
use near_sdk::json_types::U128;

#[tokio::test]
async fn test_create_proposal_and_vote() -> anyhow::Result<()> {
    let sandbox = near_workspaces::sandbox().await?;

    // Compile and deploy each contract
    let dao_contract_wasm = near_workspaces::compile_project("./dao-contract").await?;
    let proposal_contract_wasm = near_workspaces::compile_project("./proposal-contract").await?;
    let token_contract_wasm = near_workspaces::compile_project("./token-contract").await?;

    let dao_contract = sandbox.dev_deploy(&dao_contract_wasm).await?;
    let proposal_contract = sandbox.dev_deploy(&proposal_contract_wasm).await?;
    let token_contract = sandbox.dev_deploy(&token_contract_wasm).await?;

    // Create accounts
    let voter_account =  sandbox.dev_create_account().await?; 
    let root_account = sandbox.dev_create_account().await?;

    // Initialize contracts 
    let _token_contract_result = token_contract
        .call("new")
        .args_json(json!({
            "owner_id": root_account.id(),
            "total_supply": "1000000000000000000000000",
            "metadata": {
                "spec": "ft-1.0.0",
                "name": "Fabri-DAO Token",
                "symbol": "FDAO",
                "icon": "data:image/svg+xml;base64,PHN2ZyB...",
                "reference": null,
                "reference_hash": null,
                "decimals": 24
            }
        }))
        .transact()
        .await?;


    let _proposal_contract_result = proposal_contract
        .call("new")
        .args_json(json!({
            "token_contract_id": token_contract.id() 
        }))
        .transact()
        .await?;

    let _dao_contract_result = dao_contract
        .call("new")
        .args_json(json!({
            "admin_account_id": root_account.id(),
            "proposal_contract_id": proposal_contract.id()
        }))
        .transact()
        .await?;

    let create_proposal_outcome = root_account
        .call(dao_contract.id(), "create_proposal")
        .args_json(serde_json::json!({
            "title": "Proposal 1",
            "description": "Description here",
            "deadline": 1714296991986877271u64,
            "options_vec": ["Yes", "No"],
            "minimum_votes": 1
        }))
        .max_gas()
        .transact()
        .await?;
    
    let proposal_id: u64 = create_proposal_outcome.json()?;

    // Issue tokens to a voter account using the token contract
    let root_balance: u128 = token_contract
        .call("ft_balance_of")
        .args_json(json!({
            "account_id": root_account.id()
        }))
        .view()
        .await?
        .json::<U128>()?.0;

    assert_eq!(root_balance, 1000000000000000000000000, "The root account does not have the correct balance.");

    let res_storage = token_contract
        .call("storage_deposit")
        .args_json(serde_json::json!({
            "account_id": voter_account.id(),
            "registration_only": true
        }))
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?; 

    assert!(res_storage.is_success(), "Storage deposit failed");

    let res_transfer = root_account
        .call(token_contract.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": voter_account.id(),
            "amount": U128(10000000000000000000000),
            "memo": "Allocating tokens for voting"
        }))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))  
        .transact()
        .await?;

    assert!(res_transfer.is_success());

    // Verify the voter account has tokens
    let voter_balance: u128 = token_contract
        .call("ft_balance_of")
        .args_json(json!({
            "account_id": voter_account.id()
        }))
        .view()
        .await?
        .json::<U128>()?.0;

    assert_eq!(voter_balance, 10000000000000000000000, "The voter account does not have the correct balance.");
    
    // Cast a vote on the proposal
    let res_vote = voter_account
        .call(proposal_contract.id(), "vote")
        .args_json(serde_json::json!({
            "proposal_id": proposal_id,
            "voter": voter_account.id(),
            "vote_option": 0
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(res_vote.is_success(), "Vote failed");
    
    // Verify the vote was cast
    let votes: Vec<(String, u64)> = proposal_contract
        .call("get_votes")
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
