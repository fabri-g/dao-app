use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::{
    FungibleToken, FungibleTokenCore, FungibleTokenResolver,
};
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::borsh::BorshSerialize;
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near, require, AccountId, BorshStorageKey, NearToken, PanicOnDefault, PromiseOrValue,
};

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjgwMCIgd2lkdGg9IjgwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB2aWV3Qm94PSIwIDAgMjk2LjQ3MyAyOTYuNDczIiB4bWw6c3BhY2U9InByZXNlcnZlIj48cGF0aCBkPSJNMTQ4LjIzNyAwQzY2LjM2OCAwIC4wMDEgNjYuMzY3LjAwMSAxNDguMjM2czY2LjM2NyAxNDguMjM2IDE0OC4yMzYgMTQ4LjIzNmM4MS44NjcgMCAxNDguMjM0LTY2LjM2NyAxNDguMjM0LTE0OC4yMzZTMjMwLjEwNCAwIDE0OC4yMzcgMHptNzMuODM4IDM4LjQ5NS05LjQ0NiAxMy45NjJhMTE0Ljc0MiAxMTQuNzQyIDAgMCAwLTQyLjMxOS0xNy41MDZsMy4zMTQtMTYuNTI1YTEzMC42NzkgMTMwLjY3OSAwIDAgMSA0OC40NTEgMjAuMDY5em0tNjYuMTcyIDE3NS41M3YxNy4yMTFoLTE0di0xNy4xMjRjLTEwLTEuMDk5LTE5LjM4NC00LjkzLTI2LjQ0LTExLjQ5NS03LjA1OC02LjU2NC0xMC4yODYtMTYuMjE1LTEwLjA1NC0yOC45NDFsLjM1OC0uNDM5aDI0LjcyNmMwIDggMS43NzkgMTMuMzExIDUuMzM2IDE2LjY2NCAzLjU1OCAzLjM1NSA4LjA4MiA1LjAzNSAxMy41NzggNS4wMzUgNS4yNjMgMCA5LjI5Ny0xLjQgMTIuMTAyLTQuMjA3IDIuODA1LTIuODA1IDQuMjA4LTYuNTc3IDQuMjA4LTExLjMyIDAtNC42ODYtMS4zNDUtOC41MDItNC4wMzQtMTEuNDUxLTIuNjg5LTIuOTUtNy4zMDMtNS42OTYtMTMuODM4LTguMjQyLTEyLjg0MS00LjY4Ni0yMi4zNzEtOS45MDQtMjguNTg4LTE1LjY2LTYuMjE4LTUuNzU1LTkuMzI2LTEzLjg2Ny05LjMyNi0yNC4zMzYgMC05LjcxOCAyLjg3My0xNy42ODQgOS4wODktMjMuOTAyIDYuMjE3LTYuMjE3IDE0Ljg4MS05Ljg3NiAyNC44ODEtMTAuOTc2VjY2LjIzNmgxNHYxOC43NzhjMTAgMS41MDUgMTguMzEgNS43NTYgMjQuMTIzIDEyLjc1NCA1LjgxMyA2Ljk5OSA4LjcyOCAxNS45NzYgOC42MTMgMjcuMTM4bC0uMTkzLjMzaC0yNC43MjdjMC03LTEuNDAzLTExLjk4NC00LjIwOC0xNS42ODYtMi44MDYtMy43MDEtNi42MzctNS41MDEtMTEuNDk1LTUuNTAxLTQuOTc2IDAtOC42NzYgMS40NTctMTEuMTA1IDQuMzItMi40MyAyLjg2My0zLjY0MyA2LjY4MS0zLjY0MyAxMS40MjMgMCA0LjUxMSAxLjI4NiA4LjE3NiAzLjg2IDEwLjk4MSAyLjU3MyAyLjgwNiA3LjMwMiA1LjU3MSAxNC4xODYgOC4yODkgMTIuNzgyIDUuMDMyIDIyLjI1NCAxMC40MTIgMjguNDEzIDE2LjEzOSA2LjE2IDUuNzI2IDkuMjQgMTMuNzM4IDkuMjQgMjQuMDMzIDAgMTAuMDY0LTMuMzYxIDE4LjEwNS05LjU0OSAyNC4xMi02LjE4OSA2LjAxNy0xNC41MTMgOS41NzMtMjUuNTEzIDEwLjY3MXpNMTIyLjg1IDE4LjQyNmwzLjE5MiAxNi41NDlhMTE0Ljc1OCAxMTQuNzU4IDAgMCAwLTQyLjMgMTcuNTVsLTkuMzQzLTE0LjAzYTEzMC42NzIgMTMwLjY3MiAwIDAgMSA0OC40NTEtMjAuMDY5ek0zOC40OTYgNzQuMzk3bDEzLjk2MiA5LjQ0NmExMTQuNzM1IDExNC43MzUgMCAwIDAtMTcuNTA1IDQyLjMxOGwtMTYuNTI2LTMuMzE0YTEzMC43MTUgMTMwLjcxNSAwIDAgMSAyMC4wNjktNDguNDV6bS0uMDAxIDE0Ny42NzZhMTMwLjY3NyAxMzAuNjc3IDAgMCAxLTIwLjA2OC00OC40NTFsMTYuNTQ5LTMuMTkyYTExNC43NjIgMTE0Ljc2MiAwIDAgMCAxNy41NSA0Mi4zMDFsLTE0LjAzMSA5LjM0MnptMzUuOTAyIDM1LjkwMyA5LjQ0Ni0xMy45NjJhMTE0LjcyNCAxMTQuNzI0IDAgMCAwIDQyLjMyIDE3LjUwN2wtMy4zMTQgMTYuNTI2YTEzMC42OSAxMzAuNjkgMCAwIDEtNDguNDUyLTIwLjA3MXptOTkuMjI2IDIwLjA3MS0zLjE5Mi0xNi41NDlhMTE0Ljc1MiAxMTQuNzUyIDAgMCAwIDQyLjMwMi0xNy41NTFsOS4zNDMgMTQuMDNhMTMwLjY4IDEzMC42OCAwIDAgMS00OC40NTMgMjAuMDd6bTg0LjM1NS01NS45NzMtMTMuOTYyLTkuNDQ2YTExNC43NDcgMTE0Ljc0NyAwIDAgMCAxNy41MDUtNDIuMzE5bDE2LjUyNSAzLjMxNGExMzAuNzEgMTMwLjcxIDAgMCAxLTIwLjA2OCA0OC40NTF6bTMuNTE5LTk2LjAzM2ExMTQuNzU5IDExNC43NTkgMCAwIDAtMTcuNTUxLTQyLjMwMmwxNC4wMy05LjM0M2ExMzAuNjgzIDEzMC42ODMgMCAwIDEgMjAuMDY5IDQ4LjQ1MmwtMTYuNTQ4IDMuMTkzeiIvPjwvc3ZnPg==";

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    FungibleToken,
    Metadata,
}

#[near]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Fabri-DAO".to_string(),
                symbol: "FDAO".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());

        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: total_supply,
            memo: Some("new tokens are minted"),
        }
        .emit();

        this
    }
}

#[near]
impl FungibleTokenCore for Contract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        self.token.ft_transfer(receiver_id, amount, memo)
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.token.ft_transfer_call(receiver_id, amount, memo, msg)
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}

#[near]
impl FungibleTokenResolver for Contract {
    #[private]
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        let (used_amount, burned_amount) =
            self.token.internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
        if burned_amount > 0 {
            log!("Account @{} burned {}", sender_id, burned_amount);
        }
        used_amount.into()
    }
}

#[near]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.token.storage_deposit(account_id, registration_only)
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        self.token.storage_withdraw(amount)
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        #[allow(unused_variables)]
        if let Some((account_id, balance)) = self.token.internal_storage_unregister(force) {
            log!("Closed @{} with {}", account_id, balance);
            true
        } else {
            false
        }
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.token.storage_balance_of(account_id)
    }
}

#[near]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_contract_standards::fungible_token::Balance;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(NearToken::from_yoctonear(1))
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(NearToken::from_near(0))
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}