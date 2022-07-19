#!/bin/zsh


INIT=$(echo $(cat init.json | jq -rc '.'))
CODE_ID=1478
echo $INIT
echo $CODE_ID
RES=$(junod tx wasm instantiate $CODE_ID \
    $INIT \
    --amount '50000ujunox'  --label "ns_flix" --from $(junod keys show -a wallet) --gas-prices '0.1ujunox' --gas auto --gas-adjustment 1.3 -b block -o json -y --no-admin) &&
echo $RES > res.json &&
CONTRACT_ADDR=$(junod query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]') &&
junod query wasm contract $CONTRACT_ADDR 
