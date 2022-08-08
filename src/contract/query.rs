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
    
}