#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"
WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

echo "Building wasm..."
cargo build

echo "Optimize wasm..."
cargo-run-script optimize

echo "Deploying on chain..."
CODE_ID=$(terrad tx wasm store ./artifacts/cw_alliance.wasm $TX_ARGS | jq -r .logs[0].events[1].attributes[1].value)

if [ "$CODE_ID" == "null" ]; then
  echo "Error: Failed to deploy the contract"
  exit 1
fi

echo "Instantiating $CODE_ID code on chain..."
SALT=$(openssl rand -hex 2)
CONTRACT_ADDR=$(terrad tx wasm instantiate2 $CODE_ID '{}' $SALT $TX_ARGS --label "cw_alliance_$SALT" --admin=$WALLET_ADDRESS | jq -r .logs[0].events[0].attributes[0].value)

if [ "$CONTRACT_ADDR" == "null" ]; then
  echo "Error: Failed to deploy the contract"
  exit 1
fi

echo $CONTRACT_ADDR > ./scripts/contract_addr.txt