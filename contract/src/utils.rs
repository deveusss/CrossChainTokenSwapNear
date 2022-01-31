use crate::interfaces::SwapToParams;

use super::*;

impl Contract {
    pub fn validate_swap_to(
        &self, 
        swap_to: &SwapToParams, 
    ) {
        assert!(
            swap_to.new_address.len() > 0,
            "New address must not be empty"
        );
        assert!(
            swap_to.second_path.len() > 0,
            "Second path must not be empty"
        );
        assert!(
            self.existing_other_blockchain.contains(&swap_to.blockchain) &&
            swap_to.blockchain != self.num_of_this_blockchain,
            "Wrong choose of blockchain"
        );
        assert!(
            swap_to.second_path[0] == 
            self.rubic_addresses.get(&swap_to.blockchain).unwrap(),
            "The first address in second path must be Rubic"
        );
    }

    pub fn validate_amount_in(&self, amount_in: &U128) {
        assert!(
            u128::from(*amount_in) >= self.min_token_amount,
            "Not enough tokens",
        );
        assert!(
            u128::from(*amount_in) <= self.max_token_amount,
            "Too much tokens requested",
        )
    }

    pub fn validate_token_in_is_transfer(&self, token_in: &AccountId) {
        assert_eq!(
            *token_in,
            self.get_transfer_token(),
            "Wrong transfer token",
        );
    }

    pub fn validate_token_in_is_not_transfer(&self, token_in: &AccountId) {
        assert_ne!(
            *token_in,
            self.get_transfer_token(),
            "Token in must not be a transfer token with msg provided",
        );
    }

    pub fn validate_swap_actions(&self, swap_actions: &Vec<SwapAction>) {
        assert!(
            swap_actions.len() > 0,
            "Firt path must not be empty"
        );
        let swaps_len = swap_actions.len();
        let min_amount_out = 
            swap_actions[swaps_len-1].min_amount_out;
        let token_out = 
            swap_actions[swaps_len-1].token_out.clone();
        
        assert!(
            u128::from(min_amount_out) >= self.min_token_amount,
            "Not enough tokens",
        );
        assert!(
            u128::from(min_amount_out) <= self.max_token_amount,
            "Too much tokens requested",
        );
        assert!(
            token_out == self.transfer_token,
            "Last token in first path must be Rubic"
        );
    }

    pub fn validate_swap_from(&self, swap_from: &SwapFromParams) {
        assert!(
            u128::from(swap_from.amount_in_with_fee) >= 
            self.min_token_amount,
            "Not enough tokens",
        );
        assert!(
            u128::from(swap_from.amount_in_with_fee) <= 
            self.max_token_amount,
            "Too much tokens requested",
        );
    }
} 