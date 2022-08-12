use crate::{ContractError, CONFIG};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

///
/// ```json
///     {
///         "change_admin" : {
///             "admin": "cosmwasm12....2qwe23"
///         }
///     }
/// ```
pub fn change_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.admin == info.sender {
        config.admin = new_admin;
        CONFIG.save(deps.storage, &config)?;
        Ok(Response::default())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::testing::{assert_config_state, mock_init_no_price};
    use crate::{execute, Config, ExecuteMsg};
    use cosmwasm_std::coin;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn change_admin_works() {
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

        let execute_msg = ExecuteMsg::ChangeAdmin {
            admin: "alice_key".to_owned(),
        };
        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            execute_msg,
        )
        .expect("Unexpected error.");
    }

    #[test]
    fn change_admin_fails_on_unauthorized_request() {
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

        let execute_msg = ExecuteMsg::ChangeAdmin {
            admin: "alice_key".to_owned(),
        };
        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("alice_key", &[]),
            execute_msg,
        )
        .expect_err("Test Passed Successfully throwing an error");
    }

    #[test]
    fn change_admin_fails_on_unauthorized_request2() {
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

        let execute_msg = ExecuteMsg::ChangeAdmin {
            admin: "alice_key".to_owned(),
        };

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("bob_key", &[]),
            execute_msg,
        )
        .expect_err("Test Passed");
    }
}
