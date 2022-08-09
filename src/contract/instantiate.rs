use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use crate::msg::{InstantiateMsg};
use crate::error::ContractError;
use crate::config::CONFIG;
use crate::{Cw721InstantiateMsg, Cw721Contract};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(deps.storage, &msg.into());

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

#[cfg(test)]
mod tests {
    use crate::Config;
    use cosmwasm_std::coin;
    use cosmwasm_std::testing::mock_dependencies;

    use crate::helpers::testing::{assert_config_state, mock_init_no_price, mock_init_with_price};

    #[test]
    pub fn proper_init_no_fees() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox"),
            },
        );
    }

    #[test]
    pub fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        mock_init_with_price(deps.as_mut(), coin(3, "token"), coin(4, "token"));

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: coin(3, "token"),
                transfer_price: coin(4, "token"),
            },
        );
    }

}