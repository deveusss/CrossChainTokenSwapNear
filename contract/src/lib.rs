use std::convert::TryInto;

use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::collections::LookupMap;
use near_sdk::{
    env, near_bindgen, ext_contract, Promise,
    AccountId, Balance, Gas, PromiseResult,
    BorshStorageKey, PanicOnDefault,
};

pub use crate::ref_finance_swap_action::{
    Action, SwapAction, RefFinanceReceiverMessage
};

mod ref_finance_swap_action;
mod errors;
mod token_receiver;
mod views;

pub const GAS_FOR_FT_TRANSFER: Gas =      30_000_000_000_000;
pub const GAS_FOT_FT_TRANSFER_CALL: Gas = 35_000_000_000_000;
pub const GAS_FOR_CALLBACK: Gas =         45_000_000_000_000;
pub const GAS_FOR_SWAP: Gas =             30_000_000_000_000;

#[ext_contract(ext_ref)]
pub trait ExtRefFinanceContract {
    fn swap(
        &mut self,
        actions: Vec<SwapAction>,
        referral_id: Option<ValidAccountId>,
    ) -> U128;
    fn withdraw(
        &mut self,
        token_id: ValidAccountId,
        amount: U128,
        unregister: Option<bool>,
    ) -> Promise;
}

#[ext_contract(ext_self)]
pub trait AfterSwap {
    fn callback_after_swap_to(
        &mut self,
        sender_id: AccountId,
        token_in: AccountId,
        amount_in: U128,
        min_amount_out: U128,
    ) -> Promise;
}

pub trait AfterSwap {
    fn callback_after_swap_to(
        &mut self,
        sender_id: AccountId,
        token_in: AccountId,
        amount_in: U128,
        min_amount_out: U128,
    ) -> Promise;
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    RbcAddresses,
    ExistingOther,
    FeeAmount,
    CryptoFee,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")] 
pub struct SwapFromParams {
    new_address: AccountId,
    token_out: AccountId,
    amount_in_without_fee: U128,
    amount_out_min: U128,
    original_tx_hash: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    manager: AccountId,
    relayer: AccountId,
    transfer_token: AccountId,
    blockchain_router: AccountId,
    num_of_this_blockchain: u64,
    rubic_addresses: LookupMap<u64, String>,
    existing_other_blockchain: LookupMap<u64, bool>,
    fee_amount_of_blockchain: LookupMap<u64, u64>,
    blockchain_crypto_fee: LookupMap<u64, U128>,
    is_running: bool,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId, 
        manager_id: ValidAccountId,
        relayer_id: ValidAccountId,
        transfer_token: ValidAccountId,
        blockchain_router: ValidAccountId,
        num_of_this_blockchain: u64,
        is_running: bool,
    ) -> Self {
        Self {
            owner: owner_id.as_ref().clone(),
            manager: manager_id.as_ref().clone(),
            relayer: relayer_id.as_ref().clone(),
            transfer_token: transfer_token.as_ref().clone(),
            blockchain_router: blockchain_router.as_ref().clone(),
            num_of_this_blockchain,
            rubic_addresses: LookupMap::new(StorageKey::RbcAddresses),
            existing_other_blockchain: LookupMap::new(StorageKey::ExistingOther),
            fee_amount_of_blockchain: LookupMap::new(StorageKey::FeeAmount),
            blockchain_crypto_fee: LookupMap::new(StorageKey::CryptoFee),
            is_running,
        }
    }

    #[payable]
    pub fn set_transfer_token(
        &mut self,
        new_transfer_token: ValidAccountId,
    ) -> bool {
        self.transfer_token = new_transfer_token.try_into().unwrap();

        true
    }

    /// Transfer tokens to end user in current blockchain
    /// * `params` - struct SwapFromParams
    /// * `msg` - string with RefFinanceReceiverMessage. 
    ///             If _None_ that user will get `transfer_token`         
    #[payable]
    pub fn swap_tokens_to_user_with_fee(
        &mut self,
        params: SwapFromParams,
        msg: Option<String>,
    ) -> Promise {
        self.assert_contract_running();
        //TODO: self.assert_predecessor_is_relayer();
        //TODO: add validate SwapFromParams

        match msg {
            Some(ref_finance_receiver_msg) => {
                ext_fungible_token::ft_transfer_call(
                    self.get_blockchain_router(),
                    params.amount_in_without_fee,
                    None,
                    ref_finance_receiver_msg,
                    &self.transfer_token,
                    1,
                    //GAS_FOT_FT_TRANSFER_CALL,
                    90_000_000_000_000,
                )
                .then(ext_fungible_token::ft_transfer(
                    params.new_address.to_string(),
                    params.amount_out_min,
                    None,
                    &params.token_out.to_string(),
                    1,
                    GAS_FOR_FT_TRANSFER,
                ))
            },
            None => {
                ext_fungible_token::ft_transfer(
                    params.new_address,
                    params.amount_out_min,
                    None,
                    &self.transfer_token,
                    1,
                    GAS_FOR_FT_TRANSFER,
                )
            }
        }
    }
}

#[near_bindgen]
impl AfterSwap for Contract {
    #[private]
    fn callback_after_swap_to(
        &mut self,
        sender_id: AccountId,
        token_in: AccountId,
        amount_in: U128,
        min_amount_out: U128,
    ) -> Promise {
        assert_eq!(env::promise_results_count(), 1, "AfterSwap: Expected 1 promise result");
        
        match env::promise_result(0) {
            PromiseResult::Failed => {
                env::log(b"Swap failed");

                ext_ref::withdraw(
                    token_in.clone().try_into().unwrap(),
                    min_amount_out,
                    None,
                    &self.blockchain_router,
                    1,
                    35_000_000_000_000,
                )
                .then(ext_fungible_token::ft_transfer(
                    sender_id,
                    min_amount_out,
                    None,
                    &token_in,
                    1,
                    GAS_FOT_FT_TRANSFER_CALL,
                ))
            }
            PromiseResult::Successful(_) => {
                env::log(b"SwapToOtherBlockchain");

                ext_ref::withdraw(
                    self.get_transfer_token().try_into().unwrap(),
                    min_amount_out,
                    None,
                    &self.blockchain_router,
                    1,
                    35_000_000_000_000,
                )
            }
            PromiseResult::NotReady => {
                unreachable!()
            }
        }
    }
}

// Internal methods implementations 
impl Contract {
    fn assert_contract_running(&self) {
        if self.is_running == false {
            env::panic(b"Contract is on pause");
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Using a unit test to rapidly debug and iterate
        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
}
