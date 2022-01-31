near call dev-1643193012129-24813219060028 swap_tokens_to_user_with_fee \
        '{
        "params": {
                "new_address": "maxik.testnet",
                "token_out": "nusdt.ft-fin.testnet",
                "amount_in_with_fee": "10000000",
                "amount_out_min": "99",
                "original_tx_hash": "1a2b3c4d5f2"
        }
        }' \
        --accountId maxik.testnet \
        --gas 150000000000000
