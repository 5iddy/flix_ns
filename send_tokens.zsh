#!/bin/zsh

source $HOME/.config/zsh/shortcutsrc

WALLET=$(junod keys show -a $1)
DATA=$(cat send_tokens.json)
MSG=$(echo $DATA | jq -rc '.')
COIN=$(echo $DATA | jq -r '.send_tokens.amount[0].amount')
DENOM=$(echo $DATA | jq -r '.send_tokens.amount[0].denom')
AMOUNT=$(echo "${COIN}${DENOM}")
INFO=$(cat info.json)
CODE_ID=$(echo $INFO | jq -r '.code_id')
CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
echo $MSG
echo $CODE_ID

echo $WALLET
RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --amount "${AMOUNT}" --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json)
echo $RES