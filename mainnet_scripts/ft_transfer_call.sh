near call wrap.near ft_transfer_call \
        '{
        "receiver_id": "multichain.rubic-finance.near",
        "amount": "10000000000000000000000",
	"msg": "{\n\"SwapTokensToOther\": {\n\"swap_actions\": [ {\n\"pool_id\": 4,\n\"token_in\": \"wrap.near\",\n\"amount_in\": \"10000000000000000000000\",\n\"token_out\": \"dac17f958d2ee523a2206206994597c13d831ec7.factory.bridge.near\",\n\"min_amount_out\": \"116007\"\n}\n],\n\"swap_to_params\": {\n\"second_path\": [\n\"0x0000000000000000000000008AC76a51cc950d9822D68b83fE1Ad97B32Cd580d\",\n\"second_token\",\n\"third_token\"\n],\n\"min_amount_out\":\"124124512542151125125\",\n\"blockchain\":1,\n\"new_address\":\"new_address_string\",\n\"swap_to_crypto\": false,\n\"signature\":\"signature_string\"\n}\n}\n}"
        }' \
                --accountId rubic-finance.near \
                --depositYocto 1 \
                --gas 245000000000000
