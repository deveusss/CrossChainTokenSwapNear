use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseOrValue};

use crate::ref_finance_swap_action::{Action, SwapAction};
use crate::errors::*;
use super::*;

/// Message parameters to receive via token function call
/// * `ExecuteSwap` - alternative to deposit + execute ref-finance.SwapAction
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
enum TokenReceiverMessage {
    ExecuteSwap {
        // Ref-finance params
        referal_id: Option<ValidAccountId>,
        force: u8,
        actions: Vec<Action>,
        // Cross-chain SwapToParams
        blockchain: u64,
        new_address: String,
        swapToCrypto: bool,
        signature: String,
    },
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Represents SwapTokensToOtherBlockchain.
    /// Callback on receiving tokens by this contract.
    /// Swap `token_in` for `transfer_token` via ref-finance 
    /// and emit swapToOther event.
    /// * `msg` format is `TokenReceiverMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self, 
        sender_id: ValidAccountId, 
        amount: U128, 
        msg: String
    ) -> PromiseOrValue<U128> {
        //TODO: self.assert_contract_running();
        let token_in = env::predecessor_account_id();

        serde_json::from_str::<TokenReceiverMessage>(&msg)
            .and_then(|message| {
               // TODO: add validate Cross-chain params in msg
               self.internal_swap_tokens(
                   sender_id.to_string(),
                   token_in,
                   amount,
                   msg,
               );
               Ok(())
            })
            .expect(RECEIVER_WRONG_MESSAGE);
           
        PromiseOrValue::Value(U128(0))
    }
}

impl Contract {
    pub(crate) fn internal_swap_tokens(
        &mut self,
        sender_id: AccountId,
        token_in: AccountId,
        amount_in: U128,
        msg: String,
    ) -> Promise {
        ext_fungible_token::ft_transfer_call(
            REF_FINANCE_ACCOUNT_ID.to_string(),
            amount_in,
            None,
            msg,
            &token_in,
            1,
           GAS_FOT_FT_TRANSFER_CALL,
        )
        .then(ext_self::callback_after_swap_to(
            sender_id.to_string(),
            token_in,
            amount_in,
            &env::current_account_id(),
            0,
            GAS_FOR_CALLBACK,
        ))
    }

    pub(crate) fn internal_swap_tokens_v2(
        &mut self,
        sender_id: AccountId,
        token_in: AccountId,
        amount_in: U128,
        msg: String,
    ) -> Promise {
        ext_fungible_token::ft_transfer_call(
            REF_FINANCE_ACCOUNT_ID.to_string(),
            amount_in,
            None,
            "".to_string(),
            &token_in,
            1,
           GAS_FOT_FT_TRANSFER_CALL,
        )/* 
        .then(ext_self::callback_after_swap_to(
            sender_id.to_string(),
            token_in,
            amount_in,
            &env::current_account_id(),
            0,
            GAS_FOR_CALLBACK,
        ))*/
        .then(ext_ref::swap(
            vec![
                SwapAction{
                    pool_id: 35,
                    token_in: "banana.ft-fin.testnet".to_string(),
                    amount_in: Some(U128(10)),
                    token_out: "nusdt.ft-fin.testnet".to_string(),
                    min_amount_out: U128(9),
                },
            ],
            None,
            &REF_FINANCE_ACCOUNT_ID,
            0,
            GAS_FOR_SWAP,
        ))
    }
}