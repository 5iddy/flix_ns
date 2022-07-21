use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NameRecord {
    pub owner: Addr,
}

pub const SUFFIX: &str = ".flix";
pub const CONFIG: Item<Config> = Item::new("config");
pub const NAME_RECORDS: Map<&str, NameRecord> = Map::new("name_records");
