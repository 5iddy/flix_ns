#!/bin/zsh


INIT=$(echo $(cat init.json | jq -rc '.'))
CODE_ID=$2
WALLET_NAME=$1
WALLET=$(junod keys show -a $WALLET_NAME) 
echo From: $WALLET 
echo Init Message: $INIT 
echo Code Id: $CODE_ID
RES=$(junod tx wasm instantiate $CODE_ID \
    $INIT \
    --amount '50000ujunox'  --label "ns_flix-v0.2.0" --from $WALLET --gas-prices '0.1ujunox' --gas auto --gas-adjustment 1.3 -b block -o json -y --no-admin) &&
echo $RES > res.json &&
CONTRACT_ADDR=$(junod query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]') &&

JSONFMT='{"code_id": $code_id, "contract_address" : $contract_address}'
jq --null-input --arg code_id $CODE_ID --arg contract_address $CONTRACT_ADDR $JSONFMT | tee info.json 
junod query wasm contract $CONTRACT_ADDR 
