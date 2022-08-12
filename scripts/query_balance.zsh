#!/bin/zsh

# $1 name

source $HOME/.config/zsh/shortcutsrc
JSONFMT=$(cat query_balance.json)
INFO=$(cat info.json)
CODE_ID=$(echo $INFO | jq -r '.code_id')
CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
MSG=$(jq --null-input --arg name $1 $JSONFMT)
echo $MSG
echo $CODE_ID
RES=$(junod query wasm contract-state smart $CONTRACT_ADDR $MSG -o json | jq -C '.')
echo $RES