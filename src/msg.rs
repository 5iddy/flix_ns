use cosmwasm_std::{Coin, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Extension = Option<Empty>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Register a name for a wallet
    Register { name: String },
    
    /// Transfer the ownership to a different wallet
    TransferName { name: String, to: String },
    
    /// Send tokens to aa wallet based on their name
    SendTokens { name: String, amount: Vec<Coin> },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// ResolveRecord returns the current address that the name resolves to
    ResolveRecord { name: String },
    Config {},
    QueryBalance { name: String }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ResolveRecordResponse {
    pub name: String,
    pub address: Option<String>,
}
