near call banana.ft-fin.testnet ft_transfer_call \
        '{
        "receiver_id": "dev-1640771380774-77776523511109",
        "amount": "100",
	"msg": "{\n\"force\":0,\n\"actions\": [ {\n\"pool_id\": 35,\n\"token_in\": \"banana.ft-fin.testnet\",\n\"amount_in\": \"100\",\n\"token_out\": \"nusdt.ft-fin.testnet\",\n\"min_amount_out\": \"0\"\n}\n],\n\"blockchain\":0,\n\"new_address\":\"maxik.testnet\",\n\"swapToCrypto\":false,\n\"signature\":\"some signature\"\n}"
        }' \
                --accountId maxik.testnet \
                --depositYocto 1 \
                --gas 200000000000000
