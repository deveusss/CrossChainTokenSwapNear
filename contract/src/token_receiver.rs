use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{serde_json, PromiseOrValue};

use crate::interfaces::{
    TokenReceiverMessage,
};
use super::*;

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
                        self.validate_swap_actions(&swap_actions);
                        self.validate_swap_to(&swap_to_params);
                        self.validate_token_in_is_not_transfer(&token_in);

                        let swaps_len = swap_actions.len();
                        let min_amount_out = 
                            swap_actions[swaps_len-1].min_amount_out;

                        self.swap_tokens(
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
                        self.validate_swap_to(&swap_to_params);
                        self.validate_amount_in(&amount);
                        self.validate_token_in_is_transfer(&token_in);

                        env::log(b"SwapToOtherBlockchain");
                    },
                    _ => unreachable!()
                }

               Ok(())
            })
            .expect("Receiver - Wrong TokenReceiverMessage format");
           
        PromiseOrValue::Value(U128(0))
    }
}

impl Contract {
    pub(crate) fn swap_tokens(
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