#!/bin/bash

mkdir -p ./scripts/.metadata

SALT=$(openssl rand -hex 2)
TOKEN_DENOM=utest$SALT
MINT_AMOUNT=1000000000000

WALLET_ADDRESS=$(terrad keys show val1 --output json | jq -r .address)

TX_ARGS="--from=val1 --node=http://localhost:16657 --chain-id=test-1 --fees=100000uluna --gas=10000000 --broadcast-mode=block -o json --yes"

echo "Creating token '$TOKEN_DENOM' with denom 'factory/$WALLET_ADDRESS/$TOKEN_DENOM'"
CREATED_RES_DENOM=$(terrad tx tokenfactory create-denom $TOKEN_DENOM $TX_ARGS | jq -r '.logs[0].events[2].attributes[1].value')
if [ "$CREATED_RES_DENOM" != "factory/$WALLET_ADDRESS/$TOKEN_DENOM" ]; then
    echo "ERROR: Tokenfactory creating denom error. Expected result 'factory/$WALLET_ADDRESS/$TOKEN_DENOM', got '$CREATED_RES_DENOM'"
    exit 1
fi

echo "Minting '$MINT_AMOUNT' units of 'factory/$WALLET_ADDRESS/$TOKEN_DENOM' with '$WALLET_ADDRESS'"
MINT_RES=$(terrad tx tokenfactory mint $MINT_AMOUNT$CREATED_RES_DENOM $TX_ARGS | jq -r '.logs[0].events[2].type')
if [ "$MINT_RES" != "coinbase" ]; then
    echo "ERROR: Tokenfactory minting error. Expected result 'coinbase', got '$CREATED_RES_DENOM'"
    exit 1
fi

echo $CREATED_RES_DENOM > ./scripts/.metadata/token_denom.txt