near call banana.ft-fin.testnet ft_transfer_call \
        '{
        "receiver_id": "dev-1643103882945-96345911945340",
        "amount": "10000",
        "msg": "{\n\"SwapTransferTokensToOther\": {\n\"swap_to_params\": {\n\"second_path\": [\n\"first_token\",\n\"second_token\",\n\"third_token\"\n],\n\"min_amount_out\":\"124124512542151125125\",\n\"blockchain\":0,\n\"new_address\":\"new_address_string\",\n\"swap_to_crypto\": false,\n\"signature\":\"signature_string\"\n}\n}}"
        }' \
                --accountId maxik.testnet \
                --depositYocto 1 \
                --gas 245000000000000
