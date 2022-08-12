use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// ResolveRecord returns the current address that the name resolves to
    ResolveRecord {
        name: String,
    },
    Config {},
    Balance {
        name: String,
    },
    Approvals {
        name: String,
        include_expired: bool,
    },
    GetSaleFalg {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryResponse {
    NameRecord { name: String, address: String },
    Balance { name: String, amount: Vec<Coin> },
    SaleFlag { flag: bool },
}
