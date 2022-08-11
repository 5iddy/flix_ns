use cosmwasm_std::{entry_point, Deps, Env, Binary, StdResult, to_binary, AllBalanceResponse};
use crate::msg::{QueryMsg, ResolveRecordResponse};
use crate::helpers::sanitize_name;
use crate::config::CONFIG;
use crate::Cw721Contract;
use cw721::Cw721Query;

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { mut name } => {
            name = sanitize_name(name);
            query_resolver(deps, env, name)
        },
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::QueryBalance { name } => query_balance(deps, env, name)
    }
}

fn query_resolver(deps: Deps, env: Env, name: String) -> StdResult<Binary> {
    let address: Option<String> = match Cw721Contract::default().owner_of(deps, env, name.to_owned(), false) {
        Ok(res) => Some(res.owner),
        Err(_) => None
    };
    let resp = ResolveRecordResponse { name, address };
    to_binary(&resp)
}

fn query_balance(deps: Deps, env: Env, mut name:String) -> StdResult<Binary>{
    name = sanitize_name(name);
    match Cw721Contract::default().owner_of(deps, env, name, false) {
        Ok(record) => {
            let amount = deps.querier.query_all_balances(record.owner)?;
            to_binary(&AllBalanceResponse{amount})
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use cosmwasm_std::{coin, coins, from_binary};
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::helpers::testing::{mock_init_no_price, mock_init_with_price, mock_register_name};
    use crate::{
        query,
        QueryMsg,
        ResolveRecordResponse
    };

    #[test]
    fn returns_empty_on_query_unregistered_name() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        // querying for unregistered name results in NotFound error
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ResolveRecord {
                name: "alice".to_string(),
            },
        )
        .unwrap();
        let value: ResolveRecordResponse = from_binary(&res).unwrap();
        assert_eq!(None, value.address);
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
        let value: ResolveRecordResponse = from_binary(&res).unwrap();
        assert_eq!(Some("alice_key".to_string()), value.address);
    }

    #[test]
    fn returns_config_on_query(){
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));
        
        let res = query(
            deps.as_ref(), 
            mock_env(), 
            QueryMsg::Config {  }
        ).unwrap();

        let config: Config = from_binary(&res).unwrap();
        
        assert_eq!(config, Config {
            purchase_price:coin(150, "ujunox"), 
            transfer_price: coin(150, "ujunox"),
            sale_flag: false,
            admin: "creator".to_owned()
        });
    }


    #[test]
    fn fails_when_query_balance_of_non_existant_name(){
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));
        let _res = query(
            deps.as_ref(), 
            mock_env(),
            QueryMsg::QueryBalance { name: "alice".to_owned() }
        ).expect_err("Error Raised Successfully");
    }
    

    #[test]
    fn returns_balance_of_an_account_based_on_name(){
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(150, "ujunox"), coin(150, "ujunox"));
        mock_register_name(deps.as_mut(), "alice_key", "alice", &coins(150, "ujunox"));
        let _res = query(
            deps.as_ref(), 
            mock_env(), 
            QueryMsg::QueryBalance { name: "alice".to_owned() })
            .unwrap();
    }
}