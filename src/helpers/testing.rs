use cosmwasm_std::testing::{ mock_env, mock_info };
use cosmwasm_std::{ coins, from_binary, Coin, Deps, DepsMut};
use crate::contract::{execute, instantiate, query};
use crate::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
use crate::state::Config;

pub fn assert_name_owner(deps: Deps, name: &str, owner: &str) {
    let res = query(
        deps,
        mock_env(),
        QueryMsg::ResolveRecord {
            name: name.to_string(),
        },
    )
    .unwrap();

    let value: ResolveRecordResponse = from_binary(&res).unwrap();
    assert_eq!(Some(owner.to_string()), value.address);
    assert_eq!(name.to_string(), value.name);
}

pub fn assert_config_state(deps: Deps, expected: Config) {
    let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
    let value: Config = from_binary(&res).unwrap();
    assert_eq!(value, expected);
}

pub fn mock_init_with_price(deps: DepsMut, purchase_price: Coin, transfer_price: Coin) {
    let msg = InstantiateMsg {
        purchase_price: Some(purchase_price),
        transfer_price: Some(transfer_price),
    };

    let info = mock_info("creator", &coins(2, "token"));
    let res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
    println!("Response {res:#?}");
}

pub fn mock_init_no_price(deps: DepsMut) {
    let msg = InstantiateMsg {
        purchase_price: None,
        transfer_price: None,
    };

    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
}


pub fn mock_register_name(deps: DepsMut, key: &str, name: &str, sent: &[Coin]) {
    // alice can register an available name
    let info = mock_info(&key, sent);
    let msg = ExecuteMsg::Register {
        name: name.to_string(),
    };
    let _res = execute(deps, mock_env(), info, msg)
        .expect("contract successfully handles Register message");
}
