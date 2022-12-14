use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise, PromiseOrValue,
};

pub const FT_TRANSFER_GAS: Gas = 10_000_000_000_000;
pub const WITHDRAW_CALLBACK_GAS: Gas = 10_000_000_000_000;
pub const FAUCET_CALLBACK_GAS: Gas = 10_000_000_000_000;

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_ft_contract)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait ExtStakingContract {
    fn ft_transfer_callback(&mut self, amount: U128, account_id: AccountId);
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct MyContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub total_balance_share: Balance,
    pub total_shared: Balance,
    pub total_account_shared: Balance,
    pub accounts: LookupMap<AccountId, Balance>,
    pub max_share_per_account: Balance,
    pub is_paused: bool,
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FaucetInfo {
    pub total_balance_share: U128,
    pub total_shared: U128,
    pub total_account_shared: U128,
    pub max_share_per_account: U128,
    pub is_paused: bool,
}

#[near_bindgen]
impl MyContract {
    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, max_share: U128) -> Self {
        MyContract {
            owner_id,
            ft_contract_id,
            total_balance_share: 0,
            total_shared: 0,
            total_account_shared: 0,
            accounts: LookupMap::new(StorageKey::AccountKey),
            max_share_per_account: max_share.0,
            is_paused: false,
        }
    }

    #[payable]
    pub fn faucet_token(&mut self, amount: U128) -> Promise {
        assert!(
            env::attached_deposit() > 1,
            "Deposit cannot greater than 1 Yocto!"
        );
        assert!(
            self.total_balance_share >= amount.0,
            "Not enough token to be shared!"
        );
        assert!(!self.is_paused, "Faucet contract was paused!");
        let account_id: AccountId = env::predecessor_account_id();
        let account_balance: Balance = self.accounts.get(&account_id).unwrap_or_else(|| 0);
        assert!(
            account_balance + amount.0 <= self.max_share_per_account,
            "Invalid amount of token!"
        );

        ext_ft_contract::ft_transfer(
            account_id.clone(),
            amount,
            Some(String::from("Faucet of ICB Token")),
            &self.ft_contract_id,
            1,
            FT_TRANSFER_GAS,
        )
        .then(ext_self::ft_transfer_callback(
            amount,
            account_id.clone(),
            &env::current_account_id(),
            0,
            FAUCET_CALLBACK_GAS,
        ))
    }

    #[private]
    pub fn ft_transfer_callback(&mut self, amount: U128, account_id: AccountId) {
        let mut account_balance: Balance = self.accounts.get(&account_id).unwrap_or_else(|| 0);
        if account_balance == 0 {
            self.total_account_shared += 1;
        }

        account_balance += amount.0;

        self.accounts.insert(&account_id, &account_balance);
        self.total_shared += amount.0;
        self.total_balance_share -= amount.0;
    }

    pub fn update_max_share(&mut self, max_share: U128) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can do this operation!"
        );
        self.max_share_per_account = max_share.0;
    }

    pub fn get_faucet_info(&self) -> FaucetInfo {
        FaucetInfo {
            total_balance_share: U128(self.total_balance_share),
            total_shared: U128(self.total_shared),
            total_account_shared: U128(self.total_account_shared),
            max_share_per_account: U128(self.max_share_per_account),
            is_paused: self.is_paused,
        }
    }

    pub fn get_shared_balance_of(&self, account_id: AccountId) -> U128 {
        let balance: Balance = self.accounts.get(&account_id).unwrap_or_else(|| 0);
        U128(balance)
    }

    pub fn pause_faucet(&mut self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can do this operation!"
        );
        self.is_paused = true;
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for MyContract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            sender_id, self.owner_id,
            "Only contract's owner can send token!"
        );
        assert_eq!(
            env::predecessor_account_id(),
            self.ft_contract_id,
            "Invalid fungible token contract ID!"
        );

        self.total_balance_share += amount.0;

        PromiseOrValue::Value(U128(0))
    }
}
