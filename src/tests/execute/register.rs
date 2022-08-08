use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, coin};
use crate::tests::helpers::{
    mock_init_no_price,
    mock_register_name,
    mock_init_with_price,
    assert_name_owner
};
use crate::{
    ExecuteMsg
};
use crate::contract::execute;
use crate::ContractError;

