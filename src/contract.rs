use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, BankMsg, Addr, AllBalanceResponse
};

use crate::coin_helpers::{assert_sent_sufficient_coin, assert_sent_sufficient_coins};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
use crate::state::{Config, NameRecord, CONFIG, NAME_RECORDS, SUFFIX};

const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config_state = Config {
        purchase_price: msg.purchase_price,
        transfer_price: msg.transfer_price,
    };

    CONFIG.save(deps.storage, &config_state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
        ExecuteMsg::TransferName { name, to } => execute_transfer(deps, env, info, name, to),
        ExecuteMsg::SendTokens { name, amount } => execute_send_tokens(deps, env, info, name, amount)
    }
}

pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mut name: String,
) -> Result<Response, ContractError> {
    name = sanitize_name(name);
    
    // we only need to check here - at point of registration
    validate_name(&name)?;
    let config_state = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config_state.purchase_price)?;

    let record = NameRecord { owner: info.sender };

    if (NAME_RECORDS.may_load(deps.storage, &name)?).is_some() {
        // name is already taken
        return Err(ContractError::NameTaken { name });
    }

    // name is available
    NAME_RECORDS.save(deps.storage, &name, &record)?;

    Ok(Response::default())
}

pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mut name: String,
    to: String,
) -> Result<Response, ContractError> {
    let config_state = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config_state.transfer_price)?;
    
    name = sanitize_name(name);
    
    let new_owner = deps.api.addr_validate(&to)?;

    NAME_RECORDS.update(deps.storage, &name, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;

    Ok(Response::default())
}

fn execute_send_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mut name: String,
    amount: Vec<Coin>
) -> Result<Response, ContractError>{
    name = sanitize_name(name);

    assert_sent_sufficient_coins(&info.funds, &amount)?;

    let to_address: Addr = if let Some(record) = NAME_RECORDS.may_load(deps.storage, &name)? {
        record.owner
    } else {
        return Err(ContractError::NameNotExists { name });
    };

    if info.sender == to_address {
        Err(ContractError::InvalidToAddress { to_address: to_address.to_string() })
    } else {
        Ok(
            Response::new()
                .add_attribute("action", "send_tokens")
                .add_attribute("from", &info.sender)
                .add_attribute("to", &to_address)
                .add_message(
                    BankMsg::Send { 
                        to_address: to_address.to_string(), 
                        amount
                    }
                )
        )
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { mut name } => {
            name = sanitize_name(name);
            query_resolver(deps, env, name)
        },
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::QueryBalance { name } => query_balance(deps, name)
    }
}

fn query_resolver(deps: Deps, _env: Env, name: String) -> StdResult<Binary> {
    let address = match NAME_RECORDS.may_load(deps.storage, &name)? {
        Some(record) => Some(String::from(&record.owner)),
        None => None,
    };
    let resp = ResolveRecordResponse { name, address };

    to_binary(&resp)
}

fn query_balance(deps: Deps, mut name:String) -> StdResult<Binary>{
    name = sanitize_name(name);
    let address = match NAME_RECORDS.may_load(deps.storage, &name)? {
        Some(record) => Some(String::from(&record.owner)),
        None => None,
    };
    let amount = deps.querier.query_all_balances(address.unwrap())?;
    to_binary(&AllBalanceResponse{amount})
}

// Sanitize name 
fn sanitize_name(name: String) -> String {
    if name.ends_with(&SUFFIX) {
        name.strip_suffix(&SUFFIX).unwrap().to_string()
    } else {
        name
    }
}

// let's not import a regexp library and just do these checks by hand
fn invalid_char(c: char) -> bool {
    let is_valid = c.is_digit(10) || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}

/// validate_name returns an error if the name is invalid
/// (we require 3-64 lowercase ascii letters, numbers, or . - _)
fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if (name.len() as u64) < MIN_NAME_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH,
        })
    } else if (name.len() as u64) > MAX_NAME_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_NAME_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}
