use cosmwasm_std::{Coin, Ex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw721::Expiration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Register a name for a wallet
    Register { name: String },
    
    /// Transfer the ownership to a different wallet
    TransferName { name: String, to: String },
    
    /// Send tokens to aa wallet based on their name
    SendTokens { name: String, amount: Vec<Coin> },

    /// Burn NFT
    Burn { name: String },

    /// Allows operator to transfer / send the NameNFT from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        name: String,
        expires: Option<Expiration>,
    },

    /// Remove previously granted Approval
    Revoke { spender: String, name: String },

    // /// For setting the Sale Flag
    // Sale { flag: bool }
}


