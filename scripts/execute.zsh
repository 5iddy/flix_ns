# $1 => wallet
# $2 => name
flix_burn_ns(){
    WALLET=$(junod keys show -a $1)&&
    INFO=$(cat info.json) &&
    CODE_ID=$(echo $INFO | jq -r '.code_id') &&
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address') &&
    JSONFMT=$(cat burn.json) && echo $JSONFMT &&
    MSG=$(jq --null-input --arg name $2 $JSONFMT) && echo $MSG
    RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json) &&
    echo Burning $2 successfull
}

# $1 => wallet
# $2 => name
# $3 => to wallet
flix_ns_transfer_name(){
    OWNER=$(junod keys show -a $1)
    NEW=$(junod keys show -a $3)
    JSONFMT=$(cat transfer.json) && echo $JSONFMT &&
    MSG=$(jq --null-input --arg name $2 --arg to $NEW $JSONFMT) &&
    INFO=$(cat info.json) &&
    CODE_ID=$(echo $INFO | jq -r '.code_id') &&
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address') &&
    echo $MSG &&
    junod tx wasm execute $CONTRACT_ADDR $MSG --from $OWNER --amount '1000ujunox' --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json | jq -C '.'
}

# $1 wallet
# $2 name
# $3 CODE_ID
flix_register_name(){
    JSONFMT=$(cat register.json) && echo $JSONFMT &&
    INFO=$(cat info.json) &&
    CODE_ID=$(echo $INFO | jq -r '.code_id') &&
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address') &&
    MSG=$(jq --null-input --arg name $2 $JSONFMT) &&
    WALLET=$(junod keys show -a $1) &&
    echo $MSG &&
    echo $CODE_ID &&
    echo $WALLET &&
    RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --amount '1000ujunox' --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json | jq -C '.')
}

# $1 wallet
# $2 flag
flix_ns_set_sale_flag(){
    JSONFMT=$(cat sale.json) &&
    MSG=$(jq --null-input --arg flag $2 $JSONFMT) &&
    WALLET=$(junod keys show -a $1) &&
    INFO=$(cat info.json) &&
    CODE_ID=$(echo $INFO | jq -r '.code_id') &&
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address') &&
    junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --amount '1000ujunox' --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json | jq -C '.'
}

# $1 => wallet
# $2 => name
# $3 => amount
# $4 => denom
flix_send_tokens_to_name(){
    WALLET=$(junod keys show -a $1)
    JSONFMT=$(cat send_tokens.json) && echo $JSONFMT &&
    COIN=$3
    DENOM=$4
    AMOUNT=$(echo "${COIN}${DENOM}")
    MSG=$(jq --null-input --arg name $2 --arg amount $COIN --arg denom $DENOM $JSONFMT) && echo $MSG &&
    INFO=$(cat info.json)
    CODE_ID=$(echo $INFO | jq -r '.code_id')
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address')
    echo $MSG &&
    echo $CODE_ID &&
    echo $AMOUNT &&
    echo $WALLET &&
    RES=$(junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --amount "${AMOUNT}" --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json) &&
    echo $AMOUNT sent to $2 successfully
}

# Change Admin
# $1 current admin wallet
# $2 new admin wallet
flix_ns_change_admin(){
    JSONFMT=$(cat change_admin.json) &&
    MSG=$(jq --null-input --arg admin $2 $JSONFMT)  &&
    WALLET=$(junod keys show -a $1) &&
    INFO=$(cat info.json) &&
    CODE_ID=$(echo $INFO | jq -r '.code_id') &&
    CONTRACT_ADDR=$(echo $INFO | jq -r '.contract_address') &&
    junod tx wasm execute $CONTRACT_ADDR $MSG --from $WALLET --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block --trace -o json | jq -C '.'
}