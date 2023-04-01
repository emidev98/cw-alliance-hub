#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
VALIDATOR_ADDRESS=$(terrad query staking validators --node=http://localhost:16657 --output json | jq -r .validators[0].operator_address)
CONTRACT_ADDR=$(cat ./scripts/.metadata/alliance_hub_contract_addr.txt)
CREATED_RES_DENOM=$(cat ./scripts/.metadata/token_denom.txt)

echo "Executing x/alliance MsgDelegate thru $CONTRACT_ADDR on chain..."
DELEGATE_RES=$(terrad tx wasm execute $CONTRACT_ADDR '{"msg_delegate": { }}' $TX_ARGS --amount=100$CREATED_RES_DENOM)
CODE_RES=$(echo $DELEGATE_RES | jq -r .code)

if [ "$CODE_RES" != "0" ]; then
  echo "Error: Failed to delegate $DELEGATE_RES"
  exit 1
fi

MINTED_NFTS=$(terrad query wasm contract-state smart $CONTRACT_ADDR '{ "get_config" : {} }' --node=http://localhost:16657 -o json | jq -r .data.minted_nfts)
TX_HASH=$(echo $DELEGATE_RES | jq -r .txhash)

echo $((MINTED_NFTS - 1)) > ./scripts/.metadata/minted_nft_index.txt
echo " - TX_HASH $TX_HASH"
echo " - MINTED_NFT $((MINTED_NFTS - 1))"