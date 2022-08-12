use crate::state::Config;
use crate::{execute, instantiate, query};
use crate::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Coin, Deps, DepsMut};

pub fn assert_name_owner(deps: Deps, name: &str, owner: &str) {
    let res = query(
        deps,
        mock_env(),
        QueryMsg::ResolveRecord {
            name: name.clone().to_string(),
        },
    )
    .unwrap();

    let value: QueryResponse = from_binary(&res).unwrap();
    assert_eq!(
        value,
        QueryResponse::NameRecord {
            name: name.to_owned(),
            address: owner.to_owned()
        }
    );
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
        sale_flag: None,
        admin: None,
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
        sale_flag: None,
        admin: None,
    };

    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps, mock_env(), info, msg).expect("Unexpected Error: ");
}

pub fn mock_init_with_params(
    deps: DepsMut,
    purchase_price: Option<Coin>,
    transfer_price: Option<Coin>,
    sale_flag: Option<bool>,
    admin: Option<String>,
) {
    let msg = InstantiateMsg {
        purchase_price,
        transfer_price,
        sale_flag,
        admin,
    };

    let info = mock_info("creator", &[]);
    let _res = instantiate(deps, mock_env(), info, msg).expect("Unexpected Error: ");
}

pub fn mock_register_name(deps: DepsMut, key: &str, name: &str, sent: &[Coin]) {
    let info = mock_info(&key, sent);
    let msg = ExecuteMsg::Register {
        name: name.to_string(),
    };
    let _res = execute(deps, mock_env(), info, msg).expect("Unexpected Error: ");
}
