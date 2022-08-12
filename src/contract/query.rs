use crate::config::CONFIG;
use crate::helpers::sanitize_name;
use crate::msg::{QueryMsg, QueryResponse};
use crate::Cw721Contract;
use cosmwasm_std::{entry_point, to_binary, Binary, Deps, Env, StdResult};
use cw721::Cw721Query;

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { mut name } => {
            name = sanitize_name(name);
            name_resolver(deps, env, name)
        }
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::Balance { name } => get_balance(deps, env, name),
        QueryMsg::Approvals {
            name,
            include_expired,
        } => get_approvals(deps, env, name, include_expired),
        QueryMsg::GetSaleFalg {} => get_sale_flag(deps, env),
    }
}

fn name_resolver(deps: Deps, env: Env, name: String) -> StdResult<Binary> {
    let address = Cw721Contract::default()
        .owner_of(deps, env, name.to_owned(), false)?
        .owner;
    let resp = QueryResponse::NameRecord { name, address };
    to_binary(&resp)
}

fn get_balance(deps: Deps, env: Env, mut name: String) -> StdResult<Binary> {
    name = sanitize_name(name);
    match Cw721Contract::default().owner_of(deps, env, name.clone(), false) {
        Ok(record) => {
            let amount = deps.querier.query_all_balances(record.owner)?;
            to_binary(&QueryResponse::Balance { name, amount })
        }
        Err(e) => Err(e),
    }
}

fn get_sale_flag(deps: Deps, _env: Env) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&QueryResponse::SaleFlag {
        flag: config.sale_flag,
    })
}

fn get_approvals(
    deps: Deps,
    env: Env,
    mut name: String,
    include_expired: bool,
) -> StdResult<Binary> {
    name = sanitize_name(name);

    let res = Cw721Contract::default().approvals(deps, env, name, include_expired)?;

    to_binary(&res)
}

#[cfg(test)]
mod tests {
    use crate::helpers::testing::{mock_init_no_price, mock_init_with_price, mock_register_name};
    use crate::{query, Config, QueryMsg, QueryResponse};
    use cosmwasm_std::{
        coin, coins, from_binary,
        testing::{mock_dependencies, mock_env},
    };

    #[test]
    fn returns_empty_on_query_unregistered_name() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        // querying for unregistered name results in NotFound error
        let _res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ResolveRecord {
                name: "alice".to_string(),
            },
        )
        .expect_err("must throw an error");
    }

    #[test]
    fn returns_owner_on_query_registered_name() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(150, "ujunox"));

        // querying for unregistered name results in NotFound error
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ResolveRecord {
                name: "alice".to_string(),
            },
        )
        .unwrap();
        let value: QueryResponse = from_binary(&res).unwrap();
        assert_eq!(
            value,
            QueryResponse::NameRecord {
                name: "alice".to_owned(),
                address: "alice_key".to_owned()
            }
        );
    }

    #[test]
    fn returns_config_on_query() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();

        let config: Config = from_binary(&res).unwrap();

        assert_eq!(
            config,
            Config {
                purchase_price: coin(150, "ujunox"),
                transfer_price: coin(150, "ujunox"),
                sale_flag: true,
                admin: "creator".to_owned()
            }
        );
    }

    #[test]
    fn fails_when_query_balance_of_non_existant_name() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));
        let _res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Balance {
                name: "alice".to_owned(),
            },
        )
        .expect_err("Error Raised Successfully");
    }

    #[test]
    fn returns_balance_of_an_account_based_on_name() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(150, "ujunox"));
        let _res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Balance {
                name: "alice".to_owned(),
            },
        )
        .unwrap();
    }
}
