use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseOrValue};

use crate::errors::*;
use super::*;

/// Params required by cross-chain contract
/// * `second_path` - path for token swaps in target blockchain. 
///                     First must be `transfer_token` in target blockchain.
///                     Last is `desired_token` in target blockchain.
/// * `min_amount_out` - minimum amount of `desired_token` that user wants
///                         to get in target blockchain
/// * `blockchain` - uuid of target blockchain
/// * `new_address` - user's address in target blockchain
/// * `swap_to_crypto` - _true_ if user wants to get crypto in target blockchain
/// * `signature` - method signature of dex that will be used in target 
///                 blockchain for swaps 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapToParams {
    pub second_path: Vec<String>,
    pub min_amount_out: String,
    pub blockchain: u64,
    pub new_address: String,
    pub swap_to_crypto: bool,
    pub signature: String,
}

/// Message parameters to receive via token function call
/// * `SwapTransferTokensToOther` - transfer tokens from user to pool 
///                                 and emit swapToOther event.
/// * `SwapTokensToOther` - swap `token_in` for `transfer_token` via 
///                         ref-finance and emit swapToOther evnet
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
enum TokenReceiverMessage {
    SwapTransferTokensToOther {
        swap_to_params: SwapToParams,
    },
    SwapTokensToOther {
        swap_actions: Vec<SwapAction>,
        swap_to_params: SwapToParams,
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Represents SwapTokensToOtherBlockchain.
    /// Callback on receiving tokens by this contract.
    /// Swap `token_in` for `transfer_token` via ref-finance 
    /// or just emit swapToOther event if `token_in` is `transfer_token`.
    /// * `msg` format is `TokenReceiverMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self, 
        sender_id: ValidAccountId, 
        amount: U128, 
        msg: String
    ) -> PromiseOrValue<U128> {
        self.assert_contract_running();
        let token_in = env::predecessor_account_id();

        serde_json::from_str::<TokenReceiverMessage>(&msg)
            .and_then(|message| {
                match message {
                    TokenReceiverMessage::SwapTokensToOther {
                        swap_actions,
                        swap_to_params,
                    } => {
                        // TODO: add validate Cross-chain params in msg
                        let swaps_len = swap_actions.len();
                        let min_amount_out = 
                            swap_actions[swaps_len-1].min_amount_out;

                        self.internal_swap_tokens(
                            sender_id.to_string(),
                            token_in, 
                            amount,
                            min_amount_out,
                            swap_actions,
                        );
                    },
                    TokenReceiverMessage::SwapTransferTokensToOther {
                        swap_to_params,
                    } => {
                        assert_eq!(
                            token_in,
                            self.get_transfer_token(),
                            "ERR: Receiver - Wrong transfer token",
                        );
                        // TODO: add validate Cross-chain params in msg

                        env::log(b"SwapToOtherBlockchain");
                    },
                    _ => unreachable!()
                }

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
        min_amount_out: U128,
        actions: Vec<SwapAction>,
    ) -> Promise {
        ext_fungible_token::ft_transfer_call(
            self.get_blockchain_router(),
            amount_in,
            None,
            "".to_string(),
            &token_in,
            1,
            GAS_FOT_FT_TRANSFER_CALL,
        )
        .then(ext_ref::swap(
            actions,
            None,
            &self.blockchain_router,
            0,
            GAS_FOR_SWAP,
        ))
        .then(ext_self::callback_after_swap_to(
            sender_id.to_string(),
            token_in,
            amount_in,
            min_amount_out,
            &env::current_account_id(),
            0,
            GAS_FOR_CALLBACK + 35_000_000_000_000 + 35_000_000_000_000,
        ))
    }
}