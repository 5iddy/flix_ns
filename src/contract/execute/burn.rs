use crate::error::ContractError;
use crate::helpers::{sanitize_name, verified_name_owner};

use crate::{Cw721Contract, Cw721ExecuteMsg};
use cosmwasm_std::{ DepsMut, Env, MessageInfo, Response};

/// Delete Name or NFT
/// ```json
///     {
///         "burn": {
///             "name": "string"
///         }
///     }
/// ```
pub fn burn_name_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut name: String,
) -> Result<Response, ContractError> {
    name = sanitize_name(name);

    // Get the owner's address for name/NFT
    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;
    
    // If the sender is the owner of the name/NFT
    if info.sender == owner {
        // Issue a burn request to the Cw721Contract
        let message = Cw721ExecuteMsg::Burn { token_id: name };
        match Cw721Contract::default().execute(deps, env, info, message) {
            Ok(res) => Ok(res),
            Err(e) => Err(ContractError::Cw721ContractError(e)),
        }
    } else {
        // Else they are not authorized to take the following action
        Err(ContractError::Unauthorized {})
    }
}

#[cfg(test)]
mod tests{
    use crate::{execute, ContractError};
    use crate::helpers::testing::{mock_init_with_price, mock_register_name,};
    use crate::ExecuteMsg;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::coin;

    #[test]
    fn burn_name_nft_works() {
        let mut deps = mock_dependencies();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Burn {
            name: "alice".to_string(),
        };

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => {},
            Err(e) => panic!("Error Occured: {}", e)
        };
    }

    #[test]
    fn fails_burn_name_nft_unauthorized() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let required_coin = coin(2, "token");
        let required_coins = vec![required_coin.clone()];
        mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
        mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
        mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
        let info = mock_info("alice_key", &required_coins.clone());
        let msg = ExecuteMsg::Burn {
            name: "bob".to_string(),
        };

        match execute(deps.as_mut(), env, info, msg) {
            Ok(_) => panic!("Must Throw Error"),
            Err(ContractError::Unauthorized {  }) => {},
            Err(e) => panic!("Unexpected Error Occured: {}", e)
        };
    }
}