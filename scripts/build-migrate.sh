#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"
WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

echo "Building wasm..."
cargo build

echo "Deploying on chain..."
ALLIANCE_HUB_CODE_ID=$(terrad tx wasm store ./artifacts/cw_alliance_hub.wasm $TX_ARGS | jq -r .logs[0].events[1].attributes[1].value)

if [ "$ALLIANCE_HUB_CODE_ID" == "null" ]; then
  echo "Error: Failed to deploy the contract"
  exit 1
fi

CONTRACT_ADDR=$(cat ./scripts/.metadata/alliance_hub_contract_addr.txt)
echo "Migrating $CONTRACT_ADDR to $ALLIANCE_HUB_CODE_ID code on chain..."
MIGRATE_CODE=$(terrad tx wasm migrate $CONTRACT_ADDR $ALLIANCE_HUB_CODE_ID '{ "migrate": {}}' $TX_ARGS | jq -r .code)

if [ "$MIGRATE_CODE" != "0" ]; then
  echo "Error: Failed to migrate the contract"
  exit 1
fi

echo $ALLIANCE_HUB_CODE_ID > ./scripts/.metadata/alliance_hub_code_id.txt