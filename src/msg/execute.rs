use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw721::Expiration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Register a name for a wallet
    /// 
    /// ```json
    ///     {
    ///         "register": {
    ///             "name": "alice"
    ///         }
    ///     }
    /// ```
    Register { name: String },
    
    /// Transfer the ownership to a different wallet
    /// 
    /// ```json
    ///     {
    ///         "transfer_name": {
    ///             "name": "alice",
    ///             "to": "juno12......we34ex"
    ///         }
    ///     }
    /// ```
    TransferName { name: String, to: String },
    
    /// Send tokens to aa wallet based on their name
    /// 
    /// ```json
    ///     {
    ///         "send_tokens": {
    ///             "name": "alice",
    ///             "amount": [
    ///                 {
    ///                     "amount": "1000",
    ///                     "denom": "ujunox"
    ///                 }
    ///             ]
    ///         }
    ///     }
    /// ```
    SendTokens { name: String, amount: Vec<Coin> },

    /// Burn NFT
    /// 
    /// ```json
    ///     {
    ///         "burn": {
    ///             "name": "alice"
    ///         }
    ///     }
    /// ```
    Burn { name: String },

    /// Allows operator to transfer / send the NameNFT from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    /// ```json
    ///     {
    ///         "approve": {
    ///             "spender": "cosmwasm12...wer23uwei3",
    ///             "name": "alice",
    ///             "expires": {
    ///                 "at_height": <block-height-u64>,
    ///                     // or
    ///                 "at_time": <unix-timestamp>,
    ///                     // or 
    ///                 "never": {}
    ///             }
    ///         }
    ///     }
    /// ```
    Approve {
        spender: String,
        name: String,
        expires: Option<Expiration>,
    },

    /// Remove previously granted Approval
    /// ```json
    ///     {
    ///         "revoke": {
    ///             "name": "alice",
    ///             "spender": "cosmwasm12....2wer34s"
    ///         }
    ///     }
    /// ```
    Revoke { spender: String, name: String },

    /// For setting the Sale Flag
    /// ```json
    ///     {
    ///         "set_sale": {
    ///             "flag": true
    ///         }
    ///     }
    /// ```
    SetSale { flag: bool },

    /// Change Admin
    ///
    /// ```json
    ///     {
    ///         "change_admin" : {
    ///             "admin": "cosmwasm12....2qwe23"
    ///         }
    ///     }
    /// ```
    ChangeAdmin { admin: String }
}


