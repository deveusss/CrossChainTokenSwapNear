near call dev-1640771380774-77776523511109 swap_tokens_to_user_with_fee_v2 \
        '{
        "params": {
                "new_address": "maxik.testnet",
                "token_out": "nusdt.ft-fin.testnet",
                "amount_with_fee": "10",
                "amount_out_min": "0",
                "original_tx_hash": "1a2b3c4d5f"
        },
        "actions": [{
		"pool_id": 35,
		"token_in": "banana.ft-fin.testnet",
		"amount_in": "10",
		"token_out": "nusdt.ft-fin.testnet",
		"min_amount_out": "0"
	}]
        }' \
        --accountId maxik.testnet \
	--gas 100000000000000
