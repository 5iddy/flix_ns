use crate::error::ContractError;
use crate::helpers::verified_name_owner;
use crate::{Cw721Contract, Cw721ExecuteMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw721::Expiration;

pub fn approve_spender_for_name(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    name: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;
    if info.sender == owner {
        let msg = Cw721ExecuteMsg::Approve {
            spender,
            token_id: name,
            expires,
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
    use crate::{
        execute,
        helpers::testing::{assert_config_state, mock_init_no_price},
        Config, ContractError, ExecuteMsg,
    };
    use cosmwasm_std::{
        coin, coins,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    /*
        Simple Test for Approvals
    */
    #[test]
    fn approval_works_simple() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: true,
                admin: "creator".to_owned(),
            },
        );

        // create a new registration
        let execute_msg = ExecuteMsg::Register {
            name: "alice".to_owned(),
        };
        let info = mock_info("alice_key", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg) {
            Ok(_res) => {}
            Err(e) => panic!("Unexpected Error has Occured: {}", e),
        };

        // Now approve frank_key as a spender to alice
        let execute_msg = ExecuteMsg::Approve {
            spender: "frank_key".to_owned(),
            name: "alice".to_owned(),
            expires: None,
        };
        let info = mock_info("alice_key", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg) {
            Ok(_res) => {}
            Err(e) => panic!("Unexpected Error: {}", e),
        };
    }

    /*
        Only the NFT owner should able to approve spenders
    */
    #[test]
    fn fails_approve_unauthorized_request() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: true,
                admin: "creator".to_owned(),
            },
        );

        // create a new registration
        let execute_msg = ExecuteMsg::Register {
            name: "alice".to_owned(),
        };
        let info = mock_info("alice_key", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg) {
            Ok(_res) => {}
            Err(e) => panic!("Unexpected Error has Occured: {}", e),
        };

        // Mr. Frank Key should not able to approve himself as a spender to alice
        let execute_msg = ExecuteMsg::Approve {
            spender: "frank_key".to_owned(),
            name: "alice".to_owned(),
            expires: None,
        };
        let info = mock_info("frank_key", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg) {
            Ok(res) => panic!("Must Return Error: {:#?}", res),
            Err(ContractError::Unauthorized {}) => {}
            Err(e) => panic!("Unexpected Error: {}", e),
        };
    }

    /*
    Even the admin should not be able to change approvals for NFTs
    No one except the owner should be able to change approvals.
    */
    #[test]
    fn fails_approve_for_creator() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: true,
                admin: "creator".to_owned(),
            },
        );

        // create a new registration
        let execute_msg = ExecuteMsg::Register {
            name: "alice".to_owned(),
        };
        let info = mock_info("alice_key", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg) {
            Ok(_res) => {}
            Err(e) => panic!("Unexpected Error has Occured: {}", e),
        };

        // Mr. Frank Key should not able to approve himself as a spender to alice
        let execute_msg = ExecuteMsg::Approve {
            spender: "creator".to_owned(),
            name: "alice".to_owned(),
            expires: None,
        };
        let info = mock_info("creator", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg) {
            Ok(res) => panic!("Must Return Error: {:#?}", res),
            Err(ContractError::Unauthorized {}) => {}
            Err(e) => panic!("Unexpected Error: {}", e),
        };
    }
}
