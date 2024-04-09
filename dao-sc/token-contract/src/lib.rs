use near_sdk::{
    AccountId, PanicOnDefault, borsh::{BorshDeserialize, BorshSerialize, self},
    env, json_types::U128, near_bindgen,
};
use near_sdk_contract_tools::{
    Owner, Pause, ft::*, owner::{*, hooks::OnlyOwner},
    pause::{*, hooks::PausableHook},
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Owner, Pause, FungibleToken)]
#[fungible_token(all_hooks = "PausableHook", mint_hook = "OnlyOwner")]
#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {};

        Owner::init(&mut contract, &"fabri.testnet".parse().unwrap());
        Nep148Controller::set_metadata(
            &mut contract,
            &FungibleTokenMetadata::new("Fabri-DAO".to_string(), "FDAO".to_string(), 18),
        );

        Nep141Controller::mint(
            &mut contract,
            &Nep141Mint {
                amount: 100000u128,
                receiver_id: &env::predecessor_account_id(),
                memo: None,
            },
        )
        .unwrap_or_else(|e| env::panic_str(&e.to_string()));

        contract
    }

    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        Nep141Controller::mint(
            self,
            &Nep141Mint {
                amount: amount.into(),
                receiver_id: &account_id,
                memo: None,
            },
        )
        .unwrap_or_else(|e| env::panic_str(&e.to_string()));
    }

    pub fn burn(&mut self, amount: U128) {
        Nep141Controller::burn(
            self,
            &Nep141Burn {
                amount: amount.into(),
                owner_id: &env::predecessor_account_id(),
                memo: None,
            },
        )
        .unwrap_or_else(|e| env::panic_str(&e.to_string()));
    }
}