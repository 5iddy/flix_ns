use crate::InstantiateMsg;
use cosmwasm_std::{coin, Coin, DepsMut, MessageInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Configuration State
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// The amount that must be sent to the contract during registration.
    /// This amount doesn't include gas fee. It is set when the contract is 
    /// instantiated and the prices cannot be changed once set.
    pub purchase_price: Coin,

    /// The amount that must be sent to the contract during transfer.
    /// This amount doesn't include gas fee. It is set when the contract is 
    /// instantiated and the prices cannot be changed once set.
    pub transfer_price: Coin,

    /// If sale flag is true, name can be registered. Otherwise, it cannot be
    /// registered until sale_flag is set to true. Only the admin can set the
    /// sale_flag using the [`ExecuteMsg::SetSale`](crate::ExecuteMsg::SetSale).
    pub sale_flag: bool,

    /// Admin can change the sale_flag after the instantiation process.
    /// The sale_flag can only changed by the admin. [`ExecuteMsg::SetSale`](crate::ExecuteMsg::SetSale) and [`ExecuteMsg::ChangeAdmin`](crate::ExecuteMsg::ChangeAdmin)
    /// only work when called by this address.
    pub admin: String,
}

impl Config {
    /// Easy method to validate the [InstantiateMsg] and create a Config variable
    /// ```
    /// let config: Config = Config::new(&deps, info, msg);
    /// ```
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
