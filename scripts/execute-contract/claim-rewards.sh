#/bin/bash

# TX_ARGS="--from=priv-test --node=http://18.194.243.144:26657 --chain-id=pisco-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
VALIDATOR_ADDRESS=$(terrad query staking validators --node=http://localhost:16657 --output json | jq -r .validators[0].operator_address)
CONTRACT_ADDR=$(cat ./scripts/.metadata/alliance_hub_contract_addr.txt)
MINTED_NFT_INDEX=$(cat ./scripts/.metadata/minted_nft_index.txt)

echo "Executing x/alliance MsgClaimRewards for NFT $MINTED_NFT_INDEX thru $CONTRACT_ADDR on chain..."
terrad tx wasm execute $CONTRACT_ADDR '{"msg_claim_rewards": { "token_id" : "$MINTED_NFT_INDEX" }}' $TX_ARGS | jq -r .
if [ "$CLAIM_REWARDS_CODE_RES" != "0" ]; then
  echo "Error: Failed to claim rewards"
  exit 1
fi

MINTED_NFTS=$(terrad query wasm contract-state smart terra1ds5m6wwuu0cr35kmhxmq3up2w0tamsplnna3wm0dkxnyu5x8k03s600jxt '{ "get_config" : {} }' --node=http://localhost:16657 -o json | jq -r .data.minted_nfts)
CONTRACT_ADDR=$(terrad query wasm contract-state smart terra1ds5m6wwuu0cr35kmhxmq3up2w0tamsplnna3wm0dkxnyu5x8k03s600jxt '{ "get_config" : {} }' --node=http://localhost:16657 -o json | jq -r .data.nft_contract_addr)

echo "MINTED_NFT $((MINTED_NFTS - 1)) MINTED_NFTS $MINTED_NFTS CONTRACT_ADDR $CONTRACT_ADDR"