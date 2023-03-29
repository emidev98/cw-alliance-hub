#!/bin/bash

WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)
TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"
ALLIANCE_TOKEN=$(cat ./scripts/.metadata/token_denom.txt)

echo "Creating an alliance with the denom '$ALLIANCE_TOKEN'"
PROPOSAL_HEIGHT=$(terrad tx gov submit-legacy-proposal create-alliance $ALLIANCE_TOKEN 5 0 5 0 0.99 10s $TX_ARGS --deposit=10000000000uluna | jq -r .height)
PROPOSAL_ID=$(terrad query gov proposals --node=http://localhost:16657 --output json  | jq -r .proposals[-1].id)
VOTE_RES=$(terrad tx gov vote $PROPOSAL_ID yes $TX_ARGS | jq -r .code)

if [ "$VOTE_RES" != "0" ]; then
  echo "Error: Failed to vote for the proposal '$PROPOSAL_ID'"
  exit 1
fi

echo "Proposal voted successfully, waiting 10s for it to pass..."
sleep 10