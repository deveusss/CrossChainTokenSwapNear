near call dev-1643014329889-22386061794158 swap_tokens_to_user_with_fee \
        '{
        "params": {
                "new_address": "maxik.testnet",
                "token_out": "banana.ft-fin.testnet",
                "amount_in_without_fee": "100",
                "amount_out_min": "99",
                "original_tx_hash": "1a2b3c4d5f"
        }
        }' \
        --accountId maxik.testnet \
        --gas 150000000000000
