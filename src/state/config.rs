use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{coin, Coin, MessageInfo, DepsMut};

use crate::InstantiateMsg;

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
    pub admin: String
}

impl Config {
    pub fn new<'a>(deps: &'a DepsMut, info: MessageInfo, msg:InstantiateMsg) -> Self {
        let purchase_price = match msg.purchase_price {
            Some(c) => c,
            None => coin(100, "ujunox")
        };
        
        
        let transfer_price = match msg.transfer_price {
            Some(c) => c,
            None => coin(100, "ujunox")
        };

        let sale_flag = match msg.sale_flag {
            Some(v) => v,
            None => true
        };

        let admin = match msg.admin {
            Some(v) => {
                match deps.api.addr_validate(&v) {
                    Ok(_) => v,
                    Err(_) => info.sender.to_string()
                }
            },
            None => info.sender.to_string()
        };
        
        Self { purchase_price, transfer_price, sale_flag, admin }
    }
}