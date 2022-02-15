use std::convert::TryInto;

use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{
    env, near_bindgen, ext_contract, Promise,
    AccountId, Gas, PromiseResult,
    BorshStorageKey, PanicOnDefault,
    serde_json, 
};
pub use crate::interfaces::{
    Action, SwapAction, RefFinanceReceiverMessage, SwapFromParams,
};

mod token_receiver;
mod views;
mod management;
mod interfaces;
mod utils;

pub const GAS_FOR_FT_TRANSFER_CALL_SWAP_TO: Gas = 90_000_000_000_000;
pub const GAS_FOT_FT_TRANSFER_CALL: Gas = 35_000_000_000_000;
pub const GAS_FOR_FT_TRANSFER: Gas =      30_000_000_000_000;
pub const GAS_FOR_CALLBACK_SWAP_TO: Gas = 120_000_000_000_000;
pub const GAS_FOR_CALLBACK: Gas =         45_000_000_000_000;
pub const GAS_FOR_SWAP: Gas =             30_000_000_000_000;
pub const GAS_FOR_WITHDRAW: Gas =         60_000_000_000_000;

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
    fn callback_after_swap_from(
        &mut self,
        original_tx_hash: String,
    );
}

pub trait AfterSwap {
    fn callback_after_swap_to(
        &mut self,
        sender_id: AccountId,
        token_in: AccountId,
        amount_in: U128,
        min_amount_out: U128,
    ) -> Promise;
    fn callback_after_swap_from(
        &mut self,
        original_tx_hash: String,
    );
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    RbcAddresses,
    ExistingOther,
    CryptoFee,
    ProcessedTx,
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
    min_token_amount: u128,
    max_token_amount: u128,
    acc_token_fee: u128,
    fee_amount_of_blockchain: u128,
    rubic_addresses: LookupMap<u64, String>,
    existing_other_blockchain: LookupSet<u64>,
    blockchain_crypto_fee: LookupMap<u64, U128>, // unused
    processed_tx: LookupSet<String>,
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
        min_token_amount: U128,
        max_token_amount: U128,
        fee_amount_of_blockchain: U128,
        is_running: bool,
    ) -> Self {
        Self {
            owner: owner_id.as_ref().clone(),
            manager: manager_id.as_ref().clone(),
            relayer: relayer_id.as_ref().clone(),
            transfer_token: transfer_token.as_ref().clone(),
            blockchain_router: blockchain_router.as_ref().clone(),
            num_of_this_blockchain,
            min_token_amount: u128::from(min_token_amount),
            max_token_amount: u128::from(max_token_amount),
            fee_amount_of_blockchain: u128::from(fee_amount_of_blockchain),
            acc_token_fee: 0,
            rubic_addresses: LookupMap::new(StorageKey::RbcAddresses),
            existing_other_blockchain: LookupSet::new(StorageKey::ExistingOther),
            blockchain_crypto_fee: LookupMap::new(StorageKey::CryptoFee),
            processed_tx: LookupSet::new(StorageKey::ProcessedTx),
            is_running,
        }
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
        self.assert_relayer();
        self.validate_swap_from(&params);

        assert_eq!(
            self.processed_tx.contains(&params.original_tx_hash),
            false,
            "Swap already processed",
        );

        let amount_in_without_fee = 
            u128::from(params.amount_in_with_fee) *  
            (1_000_000 - self.fee_amount_of_blockchain) / 
            1_000_000;

        self.acc_token_fee += 
            u128::from(params.amount_in_with_fee) - 
            amount_in_without_fee; 

        match msg {
            Some(ref_finance_receiver_msg) => {
                let msg = serde_json::from_str::<RefFinanceReceiverMessage>(&ref_finance_receiver_msg)
                    .and_then(|message|{
                        let msg_without_fee = match message {
                            RefFinanceReceiverMessage::ExecuteSwap { 
                                referal_id, force, mut actions 
                            } => {
                                assert!(actions.len() > 0, "Must be 1 or more SwapAction in msg");
                                let action = match actions.remove(0) {
                                    Action::Swap(swap_action) => {
                                        let mut swap_action_without_fee = swap_action;
                                        swap_action_without_fee.amount_in = Some(U128(amount_in_without_fee));

                                        Action::Swap(swap_action_without_fee)
                                    }
                                };

                                actions.push(action);

                                RefFinanceReceiverMessage::ExecuteSwap{
                                    referal_id, force, actions,
                                }
                            }
                        };

                        serde_json::to_string(&msg_without_fee)
                    })
                    .unwrap();
                // Transfer `transfer_token` to REF-FINANCE and swap them 
                // for `desired token`.
                ext_fungible_token::ft_transfer_call(
                    self.get_blockchain_router(),
                    U128(amount_in_without_fee),
                    None,
                    msg,
                    &self.transfer_token,
                    1,
                    GAS_FOR_FT_TRANSFER_CALL_SWAP_TO,
                )
                // If swap on previous tx will fail, than `transfer_token` will
                // refund to this contract. This promise also will fail.
                .then(ext_fungible_token::ft_transfer(
                    params.new_address.to_string(),
                    params.amount_out_min,
                    None,
                    &params.token_out.to_string(),
                    1,
                    GAS_FOR_FT_TRANSFER,
                ))
                .then(ext_self::callback_after_swap_from(
                    params.original_tx_hash,
                    &env::current_account_id(),
                    0,
                    GAS_FOR_CALLBACK,
                ))
            },
            None => {
                ext_fungible_token::ft_transfer(
                    params.new_address.as_ref().clone(),
                    U128(amount_in_without_fee),
                    None,
                    &self.transfer_token,
                    1,
                    GAS_FOR_FT_TRANSFER,
                )
                .then(ext_self::callback_after_swap_from(
                    params.original_tx_hash,
                    &env::current_account_id(),
                    0,
                    GAS_FOR_CALLBACK,
                ))
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
                    amount_in,
                    None,
                    &self.blockchain_router,
                    1,
                    GAS_FOR_WITHDRAW,
                )
                .then(ext_fungible_token::ft_transfer(
                    sender_id,
                    amount_in,
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
                    GAS_FOR_WITHDRAW,
                )
            }
            PromiseResult::NotReady => {
                unreachable!()
            }
        }
    }

    #[private]
    fn callback_after_swap_from(
        &mut self,
        original_tx_hash: String,
    ) {
        assert_eq!(env::promise_results_count(), 1, "AfterSwap: Expected 1 promise result");

        match env::promise_result(0) {
            PromiseResult::Failed => {
                env::log(b"SwapFromOTherBlockchain failed");
            }
            PromiseResult::Successful(_) => {
                env::log(b"SwapFromOtherBlockchain");

                self.processed_tx.insert(&original_tx_hash);
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
        assert_eq!(
            self.is_running,
            true,
            "Contract is on pause",
        );
    }

    fn assert_relayer(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.relayer,
            "Only for relayer",
        )
    }
}