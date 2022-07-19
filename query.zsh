#!/bin/zsh

source $HOME/.config/zsh/shortcutsrc
MSG=$(echo $(cat query.json | jq -rc '.'))
CODE_ID=1426
CONTRACT_ADDR=$(juno-get-contract-address-from-codeid $CODE_ID)
echo $MSG
echo $CODE_ID
RES=$(junod query wasm contract-state smart $CONTRACT_ADDR $MSG)
echo $RES