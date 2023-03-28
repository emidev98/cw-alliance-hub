#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
VALIDATOR_ADDRESS=$(terrad query staking validators --node=http://localhost:16657 --output json | jq -r .validators[0].operator_address)
CONTRACT_ADDR=$(cat ./scripts/contract_addr.txt)

echo "Executing x/alliance MsgDelegate thru $CONTRACT_ADDR on chain..."
terrad tx wasm execute $CONTRACT_ADDR '{"delegate" : {"validator_address" :"'$VALIDATOR_ADDRESS'"}}' $TX_ARGS --amount=100uluna | jq 