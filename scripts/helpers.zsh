# vim: set filetype=zsh:

juno-upload-contract(){
    WALLET=$(junod keys show -a $1)
    TX=$(junod tx wasm store $2  --from $WALLET --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --output json -y | jq -r '.txhash')
    CODE_ID=$(junod query tx $TX --output json | jq -r '.logs[0].events[-1].attributes[0].value')
    echo $CODE_ID
}

juno-get-codeid-from-tx(){
    CODE_ID=$(junod query tx $TX --output json | jq -r '.logs[0].events[-1].attributes[0].value')
    echo $CODE_ID
}

juno-get-contract-address-from-codeid(){
    CONTRACT_ADDR=$(junod query wasm list-contract-by-code $1 --output json | jq -r '.contracts[0]')
    echo $CONTRACT_ADDR
}