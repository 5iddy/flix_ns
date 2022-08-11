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
    if config.admin == info.sender.to_string() {
        config.sale_flag = sale_flag.clone();
        CONFIG.save(deps.storage, &config)?;
        Ok(
            Response::default()
                .add_attribute("action", "set_sale_flag")
                .add_attribute("sale_flag", format!("{}",sale_flag))
        )
    } else {
        Err(ContractError::Unauthorized {})
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::testing::{assert_config_state, mock_init_no_price};
    use crate::{execute, Config, ExecuteMsg, ContractError};
    use cosmwasm_std::testing::{mock_dependencies, mock_info, mock_env};
    use cosmwasm_std::coin;

    #[test]
    fn set_sale_flag_works(){
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(deps.as_ref(), Config { 
            purchase_price: coin(100, "ujunox"), 
            transfer_price: coin(100, "ujunox"), 
            sale_flag: true, 
            admin: "creator".to_owned() 
        });

        let msg = ExecuteMsg::SetSale { flag: false };
        let _res = execute(
        deps.as_mut(), 
        mock_env(), 
        mock_info("creator", &[]), 
            msg
        ).unwrap();

        assert_config_state(deps.as_ref(), Config { 
            purchase_price: coin(100, "ujunox"), 
            transfer_price: coin(100, "ujunox"), 
            sale_flag: false, 
            admin: "creator".to_owned() 
        });

        let msg = ExecuteMsg::SetSale { flag: true };
        let _res = execute(
        deps.as_mut(), 
        mock_env(), 
        mock_info("creator", &[]), 
            msg
        ).unwrap();

        assert_config_state(deps.as_ref(), Config { 
            purchase_price: coin(100, "ujunox"), 
            transfer_price: coin(100, "ujunox"), 
            sale_flag: true, 
            admin: "creator".to_owned() 
        });
    }

    #[test]
    fn fails_on_unauthorized_access(){
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(deps.as_ref(), Config { 
            purchase_price: coin(100, "ujunox"), 
            transfer_price: coin(100, "ujunox"), 
            sale_flag: true, 
            admin: "creator".to_owned() 
        });

        let msg = ExecuteMsg::SetSale { flag: false };
        match execute( deps.as_mut(), mock_env(), mock_info("not_creator", &[]), msg) {
            Ok(res) => panic!("This must be an error: {:#?}", res),
            Err(ContractError::Unauthorized {  }) => {},
            Err(e) => panic!("Enexpected Error: {}", e)
        };
    }
}