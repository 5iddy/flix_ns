use crate::config::CONFIG;
use crate::error::ContractError;
use crate::helpers::{assert_sent_sufficient_coin, sanitize_name};
use crate::{Cw721Contract, Cw721ExecuteMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::helpers::verified_name_owner;


pub fn transfer_name_nft(
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

#[cfg(test)]
mod tests {
    use crate::execute;
    use crate::helpers::testing::{
        assert_name_owner, mock_init_no_price, mock_init_with_price, mock_register_name,
    };
    use crate::ContractError;
    use crate::ExecuteMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins};

    #[test]
    fn transfer_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(100, "ujunox"));
    
        // alice can transfer her name successfully to bob
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::TransferName {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };
    
        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Transfer message");
        // querying for name resolves to correct address (bob_key)
        assert_name_owner(deps.as_ref(), "alice", "bob_key");
    }
    
    #[test]
    fn transfer_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(2, "token"));
    
        // alice can transfer her name successfully to bob
        let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
        let msg = ExecuteMsg::TransferName {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };
    
        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Transfer message");
        // querying for name resolves to correct address (bob_key)
        assert_name_owner(deps.as_ref(), "alice", "bob_key");
    }
    
    #[test]
    fn fails_on_transfer_non_existent() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(100, "ujunox"));

        // transfer "alice42" to bob fails because alice doenst own "alice42"
        let info = mock_info("alice_key", &coins(2, "token"));
        let msg = ExecuteMsg::TransferName {
            name: "alice42".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameNotExists { name }) => assert_eq!(name, "alice42"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        // querying for name resolves to correct address (alice_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }
    
    #[test]
    fn fails_on_transfer_from_nonowner() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(100, "ujunox"));
    
        // alice can transfer her name successfully to bob
        let info = mock_info("frank_key", &coins(2, "token"));
        let msg = ExecuteMsg::TransferName {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };
    
        let res = execute(deps.as_mut(), mock_env(), info, msg);
    
        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::Unauthorized { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    
        // querying for name resolves to correct address (alice_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }
    
    #[test]
    fn fails_on_transfer_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(5, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));
    
        // alice can transfer her name successfully to bob
        let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
        let msg = ExecuteMsg::TransferName {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };
    
        let res = execute(deps.as_mut(), mock_env(), info, msg);
    
        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    
        // querying for name resolves to correct address (bob_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }
}