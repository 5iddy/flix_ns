#!/bin/zsh

# $1 name
flix_ns_resolve_name(){
    JSONFMT=$(cat resolve_record.json) &&
    INFO=$(cat info.json)
    CODE_ID=$(echo $INFO | jq -r '.code_id')
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
    MSG=$(jq --null-input --arg name $1 $JSONFMT) &&
    echo $MSG &&
    echo $CODE_ID &&
    RES=$(junod query wasm contract-state smart $CONTRACT_ADDR $MSG -o json | jq -C '.') &&
    echo $RES
}


# $1 name
flix_query_balance_by_name(){
    JSONFMT=$(cat query_balance.json)
    MSG=$(jq --null-input --arg name $1 $JSONFMT)
    INFO=$(cat info.json)
    CODE_ID=$(echo $INFO | jq -r '.code_id')
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
    echo $MSG
    echo $CODE_ID
    RES=$(junod query wasm contract-state smart $CONTRACT_ADDR $MSG -o json | jq -C '.')
    echo $RES
}

# No args
flix_query_config(){
    MSG=$(cat query_config.json)
    INFO=$(cat info.json)
    CODE_ID=$(echo $INFO | jq -r '.code_id')
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
    junod query wasm contract-state smart $CONTRACT_ADDR $MSG -o json | jq -C '.'
}