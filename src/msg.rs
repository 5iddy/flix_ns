use cosmwasm_std::{Coin, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Extension = Option<Empty>;


/// The Instantiating parameters that need to be sent when the contract is
/// Instanstiated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The price for registering the name
    pub purchase_price: Option<Coin>,
    /// The price for transfering the name to a different wallet
    pub transfer_price: Option<Coin>,
    /// When the sale flag is true, people will be able to buy/register a name
    /// for their wallet
    pub sale_flag: Option<bool>,
    /// The admin will be able to toggle the sale flag
    pub admin: Option<String>
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
