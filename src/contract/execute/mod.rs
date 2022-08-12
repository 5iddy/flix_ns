use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

mod register;
use register::register_name_nft;

mod transfer;
use transfer::transfer_name_nft;

mod send_tokens;
use send_tokens::send_tokens_to_named_wallet;

mod burn;
use burn::burn_name_nft;

mod approve;
use approve::approve_spender_for_name;

mod revoke;
use revoke::revoke_spender_for_name;

mod sale;
use sale::set_sale_flag;

mod admin;
use admin::change_admin;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => register_name_nft(deps, env, info, name),
        ExecuteMsg::TransferName { name, to } => transfer_name_nft(deps, env, info, name, to),
        ExecuteMsg::SendTokens { name, amount } => {
            send_tokens_to_named_wallet(deps.as_ref(), env, info, name, amount)
        }
        ExecuteMsg::Burn { name } => burn_name_nft(deps, env, info, name),
        ExecuteMsg::Approve {
            spender,
            name,
            expires,
        } => approve_spender_for_name(deps, env, info, spender, name, expires),
        ExecuteMsg::Revoke { spender, name } => {
            revoke_spender_for_name(deps, env, info, spender, name)
        }
        ExecuteMsg::SetSale { flag } => set_sale_flag(deps, info, flag),
        ExecuteMsg::ChangeAdmin { admin } => change_admin(deps, env, info, admin),
    }
}
