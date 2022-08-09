use crate::config::CONFIG;
use crate::error::ContractError;
use crate::helpers::{
    assert_sent_sufficient_coin, assert_sent_sufficient_coins, sanitize_name, validate_name,
};
use crate::msg::ExecuteMsg;
use crate::Cw721Query;
use crate::{Cw721Contract, Cw721ContractError, Cw721ExecuteMsg, Cw721MintMsg};
use cosmwasm_std::{
    entry_point, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Response, Addr,
};
use cw721::Expiration;


/// Function that handles the registeration of a name for a wallet
/// Also mints the name as an cw721 compatible NFT with Empty Metadata Extention
fn register_name_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
) -> Result<Response, ContractError> {
    // Sanitize the requested name
    name = sanitize_name(name);

    // Validate the name, Only alphanumeric characters and a
    validate_name(&name)?;
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, Some(config.purchase_price))?;

    let msg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: name.clone(),
        owner: info.sender.to_string(),
        token_uri: None,
        extension: None,
    });

    let info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: info.funds,
    };

    match Cw721Contract::default().execute(deps, env, info, msg) {
        Ok(res) => Ok(res),
        Err(Cw721ContractError::Claimed {}) => Err(ContractError::NameTaken { name }),
        Err(e) => Err(ContractError::Cw721ContractError(e)),
    }
}

fn transfer_name_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
    to: String,
) -> Result<Response, ContractError> {
    let config_state = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, Some(config_state.transfer_price))?;

    name = sanitize_name(name);

    let new_owner = deps.api.addr_validate(&to)?;

    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;

    if info.sender == owner {
        let msg = Cw721ExecuteMsg::TransferNft {
            recipient: new_owner.to_string(),
            token_id: name.clone(),
        };
        match Cw721Contract::default().execute(deps, env, info, msg) {
            Ok(res) => Ok(res),
            Err(e) => Err(ContractError::Cw721ContractError(e)),
        }
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn send_tokens_to_named_wallet(
    deps: Deps,
    env: Env,
    info: MessageInfo,
    mut name: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    name = sanitize_name(name);

    assert_sent_sufficient_coins(&info.funds, &amount)?;

    let to_address = deps.api.addr_validate(
        Cw721Contract::default()
            .owner_of(deps, env, name, false)?
            .owner
            .as_str(),
    )?;

    if info.sender == to_address {
        Err(ContractError::InvalidToAddress {
            to_address: to_address.to_string(),
        })
    } else {
        Ok(Response::new()
            .add_attribute("action", "send_tokens")
            .add_attribute("from", &info.sender)
            .add_attribute("to", &to_address)
            .add_message(BankMsg::Send {
                to_address: to_address.to_string(),
                amount,
            }))
    }
}


/// Delete Name or NFT
fn burn_name_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
) -> Result<Response, ContractError> {
    name = sanitize_name(name);
    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;
    if info.sender == owner {
        let message = Cw721ExecuteMsg::Burn { token_id: name };
        match Cw721Contract::default().execute(deps, env, info, message) {
            Ok(res) => Ok(res),
            Err(e) => Err(ContractError::Cw721ContractError(e)),
        }
    } else {
        Err(ContractError::Unauthorized {})
    }
}

fn approve_spender_for_name(deps: DepsMut, env: Env, info: MessageInfo, spender: String, name: String, expires: Option<Expiration>) -> Result<Response, ContractError> {
    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;
    if info.sender == owner {
        let msg = Cw721ExecuteMsg::Approve { spender, token_id: name, expires };
        match Cw721Contract::default()
            .execute(deps, env, info, msg) {
            Ok(res) => Ok(res),
            Err(e) => Err(ContractError::Cw721ContractError(e))
        }
    } else {
        Err(ContractError::Unauthorized {  })
    }
}

fn revoke_spender_for_name(deps: DepsMut, env: Env, info: MessageInfo, spender: String, name: String) -> Result<Response, ContractError> {
    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;
    if info.sender == owner {
        let msg = Cw721ExecuteMsg::Revoke { spender, token_id: name };
        match Cw721Contract::default()
            .execute(deps, env, info, msg) {
            Ok(res) => Ok(res),
            Err(e) => Err(ContractError::Cw721ContractError(e))
        }
    } else {
        Err(ContractError::Unauthorized {  })
    }
}

fn verified_name_owner(deps: &DepsMut, env: Env, name: String) -> Result<Addr, ContractError> {
    let owner = match Cw721Contract::default()
        .owner_of(deps.as_ref(), env, name.clone(), false){
            Ok(res) => res.owner,
            Err(_) => return Err(ContractError::NameNotExists { name }) 
        };
    
    match deps.api.addr_validate(&owner) {
        Ok(owner) => Ok(owner),
        Err(e) => Err(ContractError::Std(e))
    }
}

#[cfg(test)]
mod tests {
    use super::execute;
    use crate::helpers::testing::{
        assert_name_owner, mock_init_no_price, mock_init_with_price, mock_register_name,
    };
    use crate::ContractError;
    use crate::ExecuteMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins};

    #[test]
    fn register_available_name_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(150, "ujunox"));

        // querying for name resolves to correct address
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

    #[test]
    fn register_available_name_and_query_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(150, "ujunox"));

        // anyone can register an available name with more fees than needed
        let info = mock_info("bob_key", &coins(5, "token"));
        let msg = ExecuteMsg::Register {
            name: "bob.flix".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Register message");

        // querying for name resolves to correct address
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
        assert_name_owner(deps.as_ref(), "bob", "bob_key");
    }

    #[test]
    fn fails_on_register_already_taken_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(150, "ujunox"));

        // bob can't register the same name
        let info = mock_info("bob_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            name: "alice.flix".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
        // alice can't register the same name again
        let info = mock_info("alice_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn register_available_name_fails_with_invalid_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        let info = mock_info("bob_key", &coins(2, "token"));

        // hi is too short
        let msg = ExecuteMsg::Register {
            name: "hi".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTooShort { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }

        // 65 chars is too long
        let msg = ExecuteMsg::Register {
            name: "01234567890123456789012345678901234567890123456789012345678901234".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTooLong { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }

        // no upper case...
        let msg = ExecuteMsg::Register {
            name: "LOUD".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::InvalidCharacter { c }) => assert_eq!(c, 'L'),
            Err(_) => panic!("Unknown error"),
        }
        // ... or spaces
        let msg = ExecuteMsg::Register {
            name: "two words".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::InvalidCharacter { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
    }

    #[test]
    fn fails_on_register_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // anyone can register an available name with sufficient fees
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn fails_on_register_wrong_fee_denom() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // anyone can register an available name with sufficient fees
        let info = mock_info("alice_key", &coins(2, "earth"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
    #[test]
    fn send_tokens_works_with_sufficient_fund() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &coins(10, "ujunox"));
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: coins(10, "ujunox"),
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => {},
            Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
        };
    }

    #[test]
    fn send_tokens_works_with_extra_fund() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &coins(15, "ujunox"));
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: coins(10, "ujunox"),
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => {},
            Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
        };
    }

    #[test]
    fn send_tokens_fails_with_insufficient_fund() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &coins(8, "ujunox"));
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: coins(10, "ujunox"),
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(r) => panic!("Expected an error: {:#?}", r),
            Err(ContractError::InsufficientFundsSent {  }) => {},
            Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
        };
    }

    #[test]
    fn send_tokens_fails_with_insufficient_funds() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &[coin(8, "ujunox"), coin(8, "ucosm")]);
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(r) => panic!("Expected an error: {:#?}", r),
            Err(ContractError::InsufficientFundsSent {  }) => {},
            Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
        };
    }

    #[test]
    fn send_tokens_fails_with_insufficient_funds2() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &[coin(8, "ujunox"), coin(10, "ucosm")]);
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(r) => panic!("Expected an error: {:#?}", r),
            Err(ContractError::InsufficientFundsSent {  }) => {},
            Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
        };
    }

    #[test]
    fn send_tokens_fails_with_insufficient_funds3() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &[coin(10, "ujunox"), coin(8, "ucosm")]);
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(r) => panic!("Expected an error: {:#?}", r),
            Err(ContractError::InsufficientFundsSent {  }) => {},
            Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
        };
    }

    #[test]
    fn send_tokens_works_with_sufficient_funds() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &[coin(10, "ujunox"), coin(10, "ucosm")]);
        let msg = ExecuteMsg::SendTokens {
            name: "bob".to_string(),
            amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => {},
            Err(e) => panic!("Unxpected an error: {:#?}", e),
        };
    }

    

    #[test]
    fn burn_name_nft() {
        let deps = mock_dependencies();
        let env = mock_env();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Burn {
            name: "bob".to_string(),
        };

        match execute(deps.as_mut(), env, info, msg) {
            Ok(res) => {},
            Err(e) => panic!("Error Occured: {}", e)
        };
    }
}
