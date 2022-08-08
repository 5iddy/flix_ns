use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::Coin;

/// The Instantiating parameters that need to be sent when the contract is
/// Instanstiated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The price for registering the name
    pub purchase_price: Option<Coin>,
    /// The price for transfering the name to a different wallet
    pub transfer_price: Option<Coin>,
    // When the sale flag is true, people will be able to buy/register a name
    // for their wallet
    // pub sale_flag: Option<bool>,
    // The admin will be able to toggle the sale flag
    // pub admin: Option<String>
}