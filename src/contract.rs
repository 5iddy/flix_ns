use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, BankMsg, AllBalanceResponse, Empty, StdError
};
use cw721::Cw721Query;

use crate::coin_helpers::{assert_sent_sufficient_coin, assert_sent_sufficient_coins};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse, Extension};
use crate::state::{Config, CONFIG, SUFFIX};

use cw721_base::{InstantiateMsg as Cw721InstantiateMsg, MintMsg, ContractError as Cw721ContractError};

pub type Cw721MintMsg = MintMsg<Extension>;
pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 64;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        purchase_price: msg.purchase_price,
        transfer_price: msg.transfer_price,
    };

    CONFIG.save(deps.storage, &config)?;

    let init_msg = Cw721InstantiateMsg {
        name: "Flix Name Service NFT".to_string(),
        symbol: "FLIXNS".to_string(),
        minter: env.contract.address.to_string(),
    };

    match Cw721Contract::default().instantiate(deps, env, info, init_msg) {
        Ok(res) => Ok(res),
        Err(e) => Err(ContractError::Std(e))
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
        ExecuteMsg::TransferName { name, to } => execute_transfer(deps, env, info, name, to),
        ExecuteMsg::SendTokens { name, amount } => execute_send_tokens(deps.as_ref(), env, info, name, amount)
    }
}

pub fn execute_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
) -> Result<Response, ContractError> {
    name = sanitize_name(name);
    
    // we only need to check here - at point of registration
    validate_name(&name)?;
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.purchase_price)?;

    let msg = Cw721ExecuteMsg::Mint(
        Cw721MintMsg {
            token_id: name.clone(),
            owner: info.sender.to_string(),
            token_uri: None,
            extension: None
        }
    );

    let info = MessageInfo { sender: env.contract.address.clone(), funds: info.funds};

    match Cw721Contract::default().execute(deps, env, info, msg) {
        Ok(res) => Ok(res),
        Err(Cw721ContractError::Claimed {  }) =>  Err(ContractError::NameTaken { name }),
        Err(e) => Err(ContractError::Cw721ContractError(e))
    }
}

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
    to: String,
) -> Result<Response, ContractError> {
    let config_state = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config_state.transfer_price)?;
    
    name = sanitize_name(name);
    
    let new_owner = deps.api.addr_validate(&to)?;
    
    let owner = match Cw721Contract::default().owner_of(deps.as_ref(), env.clone(), name.clone(), false) {
        Ok(res) => res,
        Err(e) => {
            match e {
                StdError::NotFound { kind: _ } => return Err(ContractError::NameNotExists { name: name }),
                e => return Err(ContractError::Std(e))
            }
        }
    };
    
    let owner_addr = deps.api.addr_validate(owner.owner.as_str())?;

    if info.sender == owner_addr {
        let msg = Cw721ExecuteMsg::TransferNft { recipient: new_owner.to_string(), token_id: name.clone() };
        match Cw721Contract::default().execute(deps, env, info, msg) {
            Ok(res) => Ok(res),
            Err(e) =>  Err(ContractError::Cw721ContractError(e))
        }
    } else {
        Err(ContractError::Unauthorized {  })
    }
}

fn execute_send_tokens(
    deps: Deps,
    env: Env,
    info: MessageInfo,
    mut name: String,
    amount: Vec<Coin>
) -> Result<Response, ContractError>{
    name = sanitize_name(name);

    assert_sent_sufficient_coins(&info.funds, &amount)?;

    let to_address = deps.api.addr_validate(Cw721Contract::default().owner_of(deps, env, name, false)?.owner.as_str())?;

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

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { mut name } => {
            name = sanitize_name(name);
            query_resolver(deps, env, name)
        },
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::QueryBalance { name } => query_balance(deps, env, name)
    }
}

fn query_resolver(deps: Deps, env: Env, name: String) -> StdResult<Binary> {
    let address: Option<String> = match Cw721Contract::default().owner_of(deps, env, name.to_owned(), false) {
        Ok(res) => Some(res.owner),
        Err(_) => None
    };
    let resp = ResolveRecordResponse { name, address };
    to_binary(&resp)
}

fn query_balance(deps: Deps, env: Env, mut name:String) -> StdResult<Binary>{
    name = sanitize_name(name);
    match Cw721Contract::default().owner_of(deps, env, name, false) {
        Ok(record) => {
            let amount = deps.querier.query_all_balances(record.owner)?;
            to_binary(&AllBalanceResponse{amount})
        },
        Err(e) => Err(e),
    }
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
