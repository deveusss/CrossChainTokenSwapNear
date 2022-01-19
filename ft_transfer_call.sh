near call banana.ft-fin.testnet ft_transfer_call \
        '{
        "receiver_id": "dev-1640771380774-77776523511109",
        "amount": "10",
	"msg": "{\n\"swap_actions\": [ {\n\"pool_id\": 35,\n\"token_in\": \"banana.ft-fin.testnet\",\n\"amount_in\": \"10\",\n\"token_out\": \"nusdt.ft-fin.testnet\",\n\"min_amount_out\": \"0\"\n}\n],\n\"swap_to_params\": {\n\"second_path\": [\n\"first_token\",\n\"second_token\",\n\"third_token\"\n],\n\"min_amount_out\":\"124124512542151125125\",\n\"blockchain\":0,\n\"new_address\":\"new_address_string\",\n\"swap_to_crypto\": false,\n\"signature\":\"signature_string\"\n}\n}"
        }' \
                --accountId maxik.testnet \
                --depositYocto 1 \
                --gas 245000000000000
