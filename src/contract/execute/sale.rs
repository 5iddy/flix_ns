use crate::{ContractError, CONFIG};

use cosmwasm_std::{DepsMut, MessageInfo, Response};

/// The function which is executed when SetSale variant is
/// evoked.
/// ### Example Payload for Execute::SetSale
/// ```json
///     {
///         "set_sale": {
///             "flag": true
///         }
///     }
/// ```
///
pub fn set_sale_flag(
    deps: DepsMut,
    info: MessageInfo,
    sale_flag: bool,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.admin == info.sender {
        config.sale_flag = sale_flag;
        CONFIG.save(deps.storage, &config)?;
        Ok(Response::default()
            .add_attribute("action", "set_sale_flag")
            .add_attribute("sale_flag", format!("{}", sale_flag)))
    } else {
        Err(ContractError::Unauthorized {})
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::testing::{assert_config_state, mock_init_no_price, mock_init_with_params};
    use crate::{execute, Config, ContractError, ExecuteMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins};

    #[test]
    fn set_sale_flag_works() {
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

        let msg = ExecuteMsg::SetSale { flag: false };
        let _res = execute(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: false,
                admin: "creator".to_owned(),
            },
        );

        let msg = ExecuteMsg::SetSale { flag: true };
        let _res = execute(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: true,
                admin: "creator".to_owned(),
            },
        );
    }

    #[test]
    fn fails_on_unauthorized_access() {
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

        let msg = ExecuteMsg::SetSale { flag: false };
        match execute(
            deps.as_mut(),
            mock_env(),
            mock_info("not_creator", &[]),
            msg,
        ) {
            Ok(res) => panic!("This must be an error: {:#?}", res),
            Err(ContractError::Unauthorized {}) => {}
            Err(e) => panic!("Enexpected Error: {}", e),
        };
    }

    #[test]
    fn registration_only_works_if_sale_flag_is_true() {
        let mut deps = mock_dependencies();

        mock_init_with_params(deps.as_mut(), None, None, Some(false), None);
        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: false,
                admin: "creator".to_owned(),
            },
        );

        // This should fail because the sale_flag is false and a ClosedSaleWindow error needs to be returned
        let execute_msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let info = mock_info("alice_key", &coins(100, "ujunox"));
        match execute(deps.as_mut(), mock_env(), info, execute_msg){
            Ok(res) => panic!("This must return a ContractError::ClosedSaleWindow {{ flag: bool }}\nBut got response: {:#?}", res),
            Err(ContractError::ClosedSaleWindow { flag }) => assert_eq!(flag, false),
            Err(e) => panic!("Unexpected Error Occured: {}", e)
        };

        // But if we change the flag to true
        let execute_msg = ExecuteMsg::SetSale { flag: true };
        let info = mock_info("creator", &[]);
        let _res =
            execute(deps.as_mut(), mock_env(), info, execute_msg).expect("Unexpected Error: ");
        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
                sale_flag: true,
                admin: "creator".to_owned(),
            },
        );

        // We should be able to register the name without issue
        let execute_msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let info = mock_info("alice_key", &coins(100, "ujunox"));
        let _res =
            execute(deps.as_mut(), mock_env(), info, execute_msg).expect("Unexpected Error: ");
    }
}
