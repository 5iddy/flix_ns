use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Addr, coin};
use cw_storage_plus::Item;

use crate::msg::InstantiateMsg;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub purchase_price: Coin,
    pub transfer_price: Coin,
    pub sale_flag: bool,
    pub admin: Addr
}

impl Config {
    pub fn new(msg: &InstantiateMsg, sender: Addr) -> Self {
        let purchase_price: Coin;
        let transfer_price: Coin;
        let sale_flag:bool;
        let admin: Addr;

        if msg.purchase_price == None {
            purchase_price = coin(250, "ujunox");
        } else {
            purchase_price = msg.purchase_price.clone().unwrap();
        }

        if msg.transfer_price == None {
            transfer_price = coin(250, "ujunox");
        } else {
            transfer_price = msg.transfer_price.clone().unwrap();
        }

        if msg.sale_flag == None {
            sale_flag = bool::default();
        } else {
            sale_flag = msg.sale_flag.unwrap();
        }

        if msg.admin == None {
            admin = sender;
        } else {
            admin = Addr::unchecked(msg.admin.clone().unwrap());
        }
        
        Self { purchase_price, transfer_price, sale_flag, admin }
    }
}

pub const SUFFIX: &str = ".flix";
pub const CONFIG: Item<Config> = Item::new("config");
