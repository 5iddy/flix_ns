#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{ mock_dependencies, mock_env, mock_info };
    use cosmwasm_std::{ coin, coins, from_binary, Coin, Deps, DepsMut};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
    use crate::state::Config;
}
