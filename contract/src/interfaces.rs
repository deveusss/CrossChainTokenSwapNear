use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

/// Params required by cross-chain contract for swap from other blockchain
/// * `new_address` - destination user address to transfer tokens
/// * `token_out` - address of token that user wants to receive
/// * `amount_in_without_fee - amount of tokens received in original blockchain
///                             with substracted crypto_fee
/// * `amount_out_min` - amount of tokens that user wants to receive 
///                         after swap 
/// * `original_tx_hash` - original transactions hash from other blockchain
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")] 
pub struct SwapFromParams {
    pub new_address: ValidAccountId,
    pub token_out: ValidAccountId,
    pub amount_in_without_fee: U128,
    pub amount_out_min: U128,
    pub original_tx_hash: String,
}

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
pub enum TokenReceiverMessage {
    SwapTransferTokensToOther {
        swap_to_params: SwapToParams,
    },
    SwapTokensToOther {
        swap_actions: Vec<SwapAction>,
        swap_to_params: SwapToParams,
    }
}

/// REF-FINANCE struct. Copypaste from https://github.com/ref-finance/ref-contracts/blob/audit_0.2.1/ref-exchange/src/token_receiver.rs
/// Message parametes to receive in ref-finance via token function call
/// * 'ExecuteSwap' - alternative to deposit + execute ref-finance.SwapAction
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum RefFinanceReceiverMessage {
    ExecuteSwap {
        referal_id: Option<ValidAccountId>,
        force: u8,
        actions: Vec<Action>,
    }
}

/// REF-FINANCE struct. Copypaste from https://github.com/ref-finance/ref-contracts/blob/audit_0.2.1/ref-exchange/src/action.rs
/// Single swap action
/// * `pool_id` - Pool which should be used for swapping
/// * `token_in` - Token to swap from
/// * `amount_in` - Amount to exchange.
///                 If amount_in is None, it will take amount_out 
///                 from previous step.
///                 Will fail if amount_in is None on the first step.
/// * `token_out` - Token to swap into
/// * `min_amount_out` - Required minimum amount of token_out
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapAction {
    pub pool_id: u64,
    pub token_in: AccountId,
    pub amount_in: Option<U128>,
    pub token_out: AccountId,
    pub min_amount_out: U128,
}

/// REF-FINANCE struct. Copypaste from https://github.com/ref-finance/ref-contracts/blob/audit_0.2.1/ref-exchange/src/action.rs
/// Single action. 
/// Allows to execute sequence of various actions initiated by an account.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum Action {
    Swap(SwapAction),
}