near call dev-1643103882945-96345911945340 swap_tokens_to_user_with_fee \
	'{
	"params": {
		"new_address": "maxik.testnet",
		"token_out": "nusdt.ft-fin.testnet",
		"amount_in_without_fee": "100",
		"amount_out_min": "99",
		"original_tx_hash": "1a2b3c4d5fa1"
	},
	"msg": "{\n\"force\":0,\n\"actions\": [ {\n\"pool_id\": 35,\n\"token_in\": \"banana.ft-fin.testnet\",\n\"amount_in\": \"100\",\n\"token_out\": \"nusdt.ft-fin.testnet\",\n\"min_amount_out\": \"0\"\n}\n]\n}"
	}' \
	--accountId maxik.testnet \
	--gas 190000000000000
