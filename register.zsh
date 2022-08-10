#!/bin/zsh

# $1 wallet
# $2 name

source $HOME/.config/zsh/shortcutsrc

JSONFMT=$(cat register.json)
CODE_ID=$(cat info.json | jq -r '.code_id')
CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
MSG=$(jq --null-input --arg name $2 $JSONFMT)
echo $MSG
echo $CODE_ID
WALLET=$(junod keys show -a $1)
echo $WALLET
RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --amount '1000ujunox' --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json | jq -C '.')
echo $RES