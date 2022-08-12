use crate::InstantiateMsg;
use cosmwasm_std::{coin, Coin, DepsMut, MessageInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Configuration Vars
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Purchase Price for a name
    pub purchase_price: Coin,

    /// Transfer Price for a name
    pub transfer_price: Coin,

    /// If sale flag is true, name can be purchased
    pub sale_flag: bool,

    /// The only address that can change the sale_flag
    pub admin: String,
}

impl Config {
    pub fn new(deps: &DepsMut, info: MessageInfo, msg: InstantiateMsg) -> Self {
        let purchase_price = msg.purchase_price.unwrap_or_else(|| coin(100, "ujunox"));

        let transfer_price = msg.transfer_price.unwrap_or_else(|| coin(100, "ujunox"));

        let sale_flag = msg.sale_flag.unwrap_or(true);

        let admin = match msg.admin {
            Some(v) => deps.api.addr_validate(&v).unwrap_or(info.sender),
            None => info.sender,
        };

        Self {
            purchase_price,
            transfer_price,
            sale_flag,
            admin: admin.to_string(),
        }
    }
}
