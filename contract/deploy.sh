near deploy --accountId multichain.rubic-finance.near --wasmFile target/wasm32-unknown-unknown/release/crosschain_token_swap.wasm --initFunction new --initArgs \
	'{
        "owner_id": "rubic-finance.near",
        "manager_id": "rubic-finance.near",
        "relayer_id": "rubic-finance.near",
        "transfer_token": "dac17f958d2ee523a2206206994597c13d831ec7.factory.bridge.near",
        "blockchain_router": "v2.ref-finance.near",
        "num_of_this_blockchain": 9,
        "min_token_amount": "0",
        "max_token_amount": "9999999999999999999999",
        "fee_amount_of_blockchain": "6000",
        "is_running": true
        }' \
