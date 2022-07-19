#!/bin/zsh

source $HOME/.config/zsh/shortcutsrc

MSG=$(echo $(cat register.json | jq -rc '.'))
CODE_ID=1426
CONTRACT_ADDR=$(juno-get-contract-address-from-codeid $CODE_ID)
echo $MSG
echo $CODE_ID
RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $(junod keys show -a wallet) --amount '1000ujunox' --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block -o json)
echo $RES