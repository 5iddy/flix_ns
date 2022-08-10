#!/bin/zsh

source $HOME/.config/zsh/shortcutsrc

MSG=$(echo $(cat register.json | jq -rc '.'))
CODE_ID=$(cat info.json | jq -r '.code_id')
CONTRACT_ADDR=$(juno-get-contract-address-from-codeid $CODE_ID)
echo $MSG
echo $CODE_ID
WALLET=$(junod keys show -a wallet)
echo $WALLET
RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --amount '1000ujunox' --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json)
echo $RES