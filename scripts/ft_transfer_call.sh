near call banana.ft-fin.testnet ft_transfer_call \
        '{
        "receiver_id": "dev-1643193012129-24813219060028",
        "amount": "100",
	"msg": "{\n\"SwapTokensToOther\": {\n\"swap_actions\": [ {\n\"pool_id\": 36,\n\"token_in\": \"banana.ft-fin.testnet\",\n\"amount_in\": \"100\",\n\"token_out\": \"nusdt.ft-fin.testnet\",\n\"min_amount_out\": \"90\"\n}\n],\n\"swap_to_params\": {\n\"second_path\": [\n\"first_token\",\n\"second_token\",\n\"third_token\"\n],\n\"min_amount_out\":\"124124512542151125125\",\n\"blockchain\":1,\n\"new_address\":\"new_address_string\",\n\"swap_to_crypto\": false,\n\"signature\":\"signature_string\"\n}\n}\n}"
        }' \
                --accountId maxik.testnet \
                --depositYocto 1 \
                --gas 300000000000000
