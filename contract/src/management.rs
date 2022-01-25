use super::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn set_owner(&mut self, owner_id: ValidAccountId) {
        self.assert_owner();
        self.owner = owner_id.as_ref().clone();
    }

    #[payable]
    pub fn set_manager(&mut self, manager_id: ValidAccountId) {
        self.assert_owner();
        self.manager = manager_id.as_ref().clone();
    }

    #[payable]
    pub fn set_relayer(&mut self, relayer_id: ValidAccountId) {
        self.assert_owner();
        self.relayer = relayer_id.as_ref().clone();
    }

    #[payable]
    pub fn set_transfer_token(&mut self, transfer_token: ValidAccountId) {
        self.assert_owner();
        self.transfer_token = transfer_token.as_ref().clone();
    }

    #[payable]
    pub fn set_blockchain_router(&mut self, blockchain_router: ValidAccountId) {
        self.assert_owner();
        self.blockchain_router = blockchain_router.as_ref().clone();
    }

    #[payable]
    pub fn set_num_of_this_blockchain(&mut self, num_of_this_blockchain: u64) {
        self.assert_owner();
        self.num_of_this_blockchain = num_of_this_blockchain;
    }

    #[payable]
    pub fn set_min_token_amount(&mut self, min_token_amount: U128) {
        self.assert_owner_or_manager();
        self.min_token_amount = min_token_amount;
    }

    #[payable]
    pub fn set_max_token_amount(&mut self, max_token_amount: U128) {
        self.assert_owner_or_manager();
        self.max_token_amount = max_token_amount;
    }

    #[payable]
    pub fn set_is_running(&mut self, is_running: bool) {
        self.assert_owner_or_manager();
        self.is_running = is_running;
    }

    /// OTHERS BLOCKCHAIN MANAGEMENT
    #[payable]
    pub fn add_other_blockchain(&mut self, blockchain_num: u64) {
        self.assert_owner();
        assert!(
            blockchain_num != self.num_of_this_blockchain,
            "Cannot add this blockchain to array of other blockchains"
        );
        assert!(
            self.existing_other_blockchain.insert(&blockchain_num),
            "Blockchain already added"
        )
    }

    #[payable]
    pub fn remove_other_blockchain(&mut self, blockchain_num: u64) {
        self.assert_owner();
        assert!(
            self.existing_other_blockchain.remove(&blockchain_num),
            "The blockchain was not added"
        )
    }

    /// FEE MANAGEMENT
    #[payable]
    pub fn collect_token_fee(&mut self) -> Promise {
        self.assert_owner();

        ext_fungible_token::ft_transfer(
            env::predecessor_account_id(),
            self.acc_token_fee,
            None,
            &self.transfer_token,
            1,
            GAS_FOR_FT_TRANSFER,
        )
    }

    #[payable]
    pub fn pool_balancing(&mut self, amount: U128) -> Promise {
        self.assert_owner();

        ext_fungible_token::ft_transfer(
            env::predecessor_account_id(),
            amount,
            None,
            &self.transfer_token,
            1,
            GAS_FOR_FT_TRANSFER,
        )
    }

    #[payable]
    pub fn set_rubic_address_of_blockchain(
        &mut self, 
        blockchain_num: u64,
        rubic_address: String,
    ) {
        self.assert_owner_or_manager();
        self.rubic_addresses.insert(&blockchain_num, &rubic_address);
    }

    #[payable]
    pub fn set_fee_amount_of_blockchain(
        &mut self,
        blockchain_num: u64,
        fee_amount: U128,
    ) {
        self.assert_owner_or_manager();
        self.fee_amount_of_blockchain.insert(&blockchain_num, &fee_amount);
    }

    /// ACCESS CONTROL
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only for owner",
        )
    }

    pub(crate) fn assert_owner_or_manager(&self) {
        let predecessor_id = env::predecessor_account_id();
        assert!(
            predecessor_id == self.owner ||
            predecessor_id == self.manager,
            "Only for owner and manager"
        )
    }
}