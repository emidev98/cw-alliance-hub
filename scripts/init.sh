#/bin/bash

mkdir -p ./scripts/.metadata

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"
WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

echo "Building wasm..."
cargo build

echo "Optimize wasm..."
cargo-run-script optimize

echo "Deploying 'CW721 METADATA ONCHAIN' contract..."
CW721_CODE_ID=$(terrad tx wasm store ./assets/cw721_metadata_onchain.wasm $TX_ARGS | jq -r .logs[0].events[1].attributes[1].value)

if [ "$CW721_CODE_ID" == "null" ]; then
  echo "Error: Failed to deploy 'cw721_metadata_onchain' contract '$CW721_CODE_ID'"
  exit 1
fi

echo "Deploying 'CW ALLIANCE HUB' contract..."
ALLIANCE_HUB_CODE_ID=$(terrad tx wasm store ./artifacts/cw_alliance_hub.wasm $TX_ARGS | jq -r .logs[0].events[1].attributes[1].value)

if [ "$ALLIANCE_HUB_CODE_ID" == "null" ]; then
  echo "Error: Failed to deploy 'cw_alliance_hub' contract '$ALLIANCE_HUB_CODE_ID'"
  exit 1
fi

echo "Instantiating Alliance Hub with $ALLIANCE_HUB_CODE_ID code on chain..."
SALT=$(openssl rand -hex 2)
ALLIANCE_HUB_CONTRACT_ADDR=$(terrad tx wasm instantiate $ALLIANCE_HUB_CODE_ID '{
  "cw721_code_id": '$CW721_CODE_ID',
  "cw721_collection": {
    "name": "Alliance NFT Collection '$SALT'",
    "symbol": "ALLIANCE"
  }
}' $TX_ARGS --label "cw_alliance_hub_$SALT" --admin=$WALLET_ADDRESS | jq -r .logs[0].events[0].attributes[0].value)

if [ "$ALLIANCE_HUB_CONTRACT_ADDR" == "null" ]; then
  echo "Error: Failed to instantiate 'cw_alliance_hub' contract '$ALLIANCE_HUB_CONTRACT_ADDR'"
  exit 1
fi

echo "Contracts deployed successfully"
echo "- CW721_CODE_ID: $CW721_CODE_ID"
echo "- ALLIANCE_HUB_CODE_ID: $ALLIANCE_HUB_CODE_ID"
echo "- ALLIANCE_HUB_CONTRACT_ADDR: $ALLIANCE_HUB_CONTRACT_ADDR"


echo $CW721_CODE_ID > ./scripts/.metadata/cw721_code_id.txt
echo $ALLIANCE_HUB_CODE_ID > ./scripts/.metadata/alliance_hub_code_id.txt
echo $ALLIANCE_HUB_CONTRACT_ADDR > ./scripts/.metadata/alliance_hub_contract_addr.txt