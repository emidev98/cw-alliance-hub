#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=1000000 --broadcast-mode=block -o json --yes"
TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=1000000 --broadcast-mode=block -o json --yes"

echo "Building wasm..."
cargo build
echo "Optimize wasm..."
cargo-run-script optimize

echo "Deploying on chain..."
CODE_ID=$(terrad tx wasm store ./artifacts/cw_alliance.wasm $TX_ARGS | jq -r .logs[0].events[1].attributes[1].value)

if [ -z "$CODE_ID" ]; then
  echo "Error: Failed to deploy the contract"
  exit 1
fi

echo "Instantiating $CODE_ID code on chain..."
terrad tx wasm migrate terra1qum2tr7hh4y7ruzew68c64myjec0dq2s2njf6waja5t0w879lutql2f5hd $CODE_ID '{}' $TX_ARGS | jq -r .txhash