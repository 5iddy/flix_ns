use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, coin};

use crate::InstantiateMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub purchase_price: Coin,
    pub transfer_price: Coin,
}

impl From<InstantiateMsg> for Config {
    fn from(msg: InstantiateMsg) -> Self {
        if msg.purchase_price.is_some() && msg.transfer_price.is_some(){
            Config {
                purchase_price: msg.purchase_price.unwrap(),
                transfer_price: msg.transfer_price.unwrap()
            }
        } else if msg.purchase_price.is_some() && msg.transfer_price.is_none() {
            Config {
                purchase_price: msg.clone().purchase_price.unwrap(),
                transfer_price: msg.purchase_price.unwrap()
            }
        } else if msg.purchase_price.is_none() && msg.transfer_price.is_some() {
            Config {
                purchase_price: msg.clone().transfer_price.unwrap(),
                transfer_price: msg.transfer_price.unwrap()
            }
        } else {
            Config {
                purchase_price: coin(100, "ujunox"),
                transfer_price: coin(100, "ujunox")
            }   
        }
    }
}

