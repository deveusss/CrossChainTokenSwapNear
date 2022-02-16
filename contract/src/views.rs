use super::*;

#[near_bindgen]
impl Contract {
    pub fn get_version(&self) -> u64 {
        1
    }

    pub fn get_owner(&self) -> AccountId { 
        self.owner.clone()
    }

    pub fn get_manager(&self) -> AccountId {
        self.manager.clone()
    }

    pub fn get_relayer(&self) -> AccountId {
        self.relayer.clone()
    }

    pub fn get_transfer_token(&self) -> AccountId {
        self.transfer_token.clone()
    }

    pub fn get_blockchain_router(&self) -> AccountId {
        self.blockchain_router.clone()
    }

    pub fn get_num_of_this_blockchain(&self) -> u64 {
        self.num_of_this_blockchain
    }

    pub fn get_min_token_amount(&self) -> U128 {
        U128(self.min_token_amount)
    }

    pub fn get_max_token_amount(&self) -> U128 {
        U128(self.max_token_amount)
    }

    pub fn get_acc_token_fee(&self) -> U128 {
        U128(self.acc_token_fee)
    }

    pub fn get_fee_amount_of_blockchain(&self) -> U128 {
        U128(self.fee_amount_of_blockchain)
    }

    pub fn get_rubic_address(&self, blockchain_num: u64) -> String {
        self.rubic_addresses
            .get(&blockchain_num)
            .expect("Wrong blockchain number")
    }

    pub fn existing_other_blockchain(&self, blockchain_num: u64) -> bool {
        self.existing_other_blockchain
            .contains(&blockchain_num)
    }

    pub fn is_processed_tx(&self, original_tx_hash: String) -> bool {
        self.processed_tx.contains(&original_tx_hash)
    }

    pub fn is_running(&self) -> bool {
        self.is_running 
    }
}