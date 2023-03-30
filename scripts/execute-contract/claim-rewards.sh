#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

CONTRACT_ADDR=$(cat ./scripts/.metadata/alliance_hub_contract_addr.txt)
MINTED_NFT_INDEX=$(cat ./scripts/.metadata/minted_nft_index.txt)

echo "Executing x/alliance MsgClaimRewards for NFT $MINTED_NFT_INDEX thru $CONTRACT_ADDR on chain..."

CLAIM_REWARDS_CODE_RES=$(terrad tx wasm execute $CONTRACT_ADDR '{"msg_claim_rewards": { "token_id" : "'$MINTED_NFT_INDEX'" }}' $TX_ARGS | jq -r .code)

if [ "$CLAIM_REWARDS_CODE_RES" != "0" ]; then
  echo "Error: Failed to claim rewards"
  exit 1
fi

echo "Claim rewards executed successfully"