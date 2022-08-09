use crate::error::ContractError;
use crate::helpers::verified_name_owner;
use crate::{Cw721Contract, Cw721ExecuteMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn revoke_spender_for_name(deps: DepsMut, env: Env, info: MessageInfo, spender: String, name: String) -> Result<Response, ContractError> {
    let owner = verified_name_owner(&deps, env.clone(), name.clone())?;
    if info.sender == owner {
        let msg = Cw721ExecuteMsg::Revoke { spender, token_id: name };
        match Cw721Contract::default()
            .execute(deps, env, info, msg) {
            Ok(res) => Ok(res),
            Err(e) => Err(ContractError::Cw721ContractError(e))
        }
    } else {
        Err(ContractError::Unauthorized {  })
    }
}