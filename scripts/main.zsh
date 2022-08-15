#!/bin/zsh

source ./env.zsh
source ./helpers.zsh
source ./init.zsh

# Upload the contract
CODE_ID=$(juno-upload-contract wallet ../target/wasm32-unknown-unknown/release/flix_ns.wasm)

# Instantiate the Contract
CONTRACT_ADDR=$(flix_init wallet $CODE_ID)

source ./execute.zsh
source ./query.zsh

# Register Few Names
flix_register_name wallet alice # wallet is alice
flix_register_name wallet2 frank # wallet2 is frank
flix_register_name wallet3 bob # wallet3 is bob


# Query Balance by Name
flix_query_balance_by_name alice
flix_query_balance_by_name frank
flix_query_balance_by_name bob

# Send tokens from frank to alice
flix_query_balance_by_name alice
flix_send_tokens_to_name wallet3 alice 10000 ujunox
flix_query_balance_by_name alice
flix_send_tokens_to_name wallet2 alice 10000 ujunox
flix_query_balance_by_name alice

# Burn names
flix_burn_ns wallet alice
flix_burn_ns wallet2 frank
flix_burn_ns wallet3 bob

# Names will become available for purchase after they are burnt.
flix_register_name wallet alice # wallet is alice
flix_register_name wallet2 frank # wallet2 is frank
flix_register_name wallet3 bob # wallet3 is bob

# Multiple names can be owned by a single wallet
flix_register_name wallet alice2
flix_register_name wallet2 frank2
flix_register_name wallet3 bob2

# Transfer Names
flix_ns_transfer_name wallet alice2 $(junod keys show -a wallet2)
flix_ns_resolve_name alice2
flix_ns_transfer_name wallet2 alice2 $(junod keys show -a wallet3)
flix_ns_resolve_name alice2

# sale flag
flix_ns_set_sale_flag wallet false
flix_query_config
flix_ns_set_sale_flag wallet true
flix_query_config

# Change Admin
flix_ns_change_admin $(junod keys show -a wallet) $(junod keys show -a wallet2)
flix_query_config
flix_ns_change_admin $(junod keys show -a wallet2) $(junod keys show -a wallet3)
flix_query_config