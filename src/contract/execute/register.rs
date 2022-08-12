use crate::config::CONFIG;
use crate::error::ContractError;
use crate::helpers::{assert_sent_sufficient_coin, sanitize_name, validate_name};
use crate::{Cw721Contract, Cw721ContractError, Cw721ExecuteMsg, Cw721MintMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Function that handles the registration of a name for a wallet
/// Also mints the name as an cw721 compatible NFT with Empty Metadata Extention
pub fn register_name_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
) -> Result<Response, ContractError> {
    // Load Config from the chain storage
    let config = CONFIG.load(deps.storage)?;

    // Check if sale_flag is true, if it is start the registration process
    if config.sale_flag {
        // Sanitize the requested name
        name = sanitize_name(name);

        // Validate the name, Only alphanumeric characters and a
        validate_name(&name)?;

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
    } else {
        // if the sale_flag is false, names cannot be registered.
        Err(ContractError::ClosedSaleWindow {
            flag: config.sale_flag,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::execute;
    use crate::helpers::testing::{
        assert_config_state, assert_name_owner, mock_init_no_price, mock_init_with_params,
        mock_init_with_price, mock_register_name,
    };
    use crate::Config;
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
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(2, "token"));

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
        let info = mock_info("bob_key", &coins(150, "ujunox"));
        let msg = ExecuteMsg::Register {
            name: "alice.flix".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(e) => panic!("Unknown error: {}", e),
        }
        // alice can't register the same name again
        let info = mock_info("alice_key", &coins(150, "ujunox"));
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
    fn registration_fails_on_false_sale_flag() {
        let mut deps = mock_dependencies();
        mock_init_with_params(
            deps.as_mut(),
            Some(coin(100, "token")),
            Some(coin(100, "token")),
            Some(false),
            Some("creator".to_string()),
        );

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "token"),
                transfer_price: coin(100, "token"),
                sale_flag: false,
                admin: "creator".to_owned(),
            },
        );

        let msg = ExecuteMsg::Register {
            name: "alice".to_owned(),
        };
        let info = mock_info("alice_key", &coins(100, "token"));

        let _res = match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => panic!("Must throw an error"),
            Err(ContractError::ClosedSaleWindow { flag }) => assert_eq!(flag, false),
            Err(e) => panic!("Unexpected error occured: {}", e),
        };
    }
}
