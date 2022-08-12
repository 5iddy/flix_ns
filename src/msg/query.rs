use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query enum variants are for distinguishing various kinds of query requests
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// ResolveRecord returns the current address that the name resolves to
    /// Refer to [QueryResponse] for response info.
    /// 
    /// ```json
    /// {
    ///     "resolve_record": {
    ///         "name": "alice"
    ///     }
    /// }
    /// ```
    ResolveRecord {
        /// Queries for the owner of this name
        name: String,
    },

    /// For querying for the current config state 
    /// 
    /// ```json
    /// {
    ///     "config" : {}
    /// }
    /// ```
    Config {},
    
    /// Querying for the balance of the owner of name
    /// ```json
    /// {
    ///     "balance": {
    ///         "name": "alice"
    ///     }
    /// }
    /// ```
    Balance {
        /// Based on the name, the owner's address will be resolved
        name: String,
    },

    /// For querying Approvals of a Name NFT
    /// ```json
    /// {
    ///     "approvals": {
    ///         "name": "alice",
    ///         "incldue_expired": flase
    ///     }
    /// }
    /// ```
    Approvals {
        /// the name that we want query approvals for
        name: String,
        /// The option to whether or not include expiration info
        include_expired: bool,
    },

    /// For querying the current value of sale flag
    /// ```json
    /// {
    ///     "get_sale_flag": {}
    /// }
    /// ```
    GetSaleFlag {},
}

/// Various Query Responses
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    /// Response for when [`QueryMsg::ResolveRecord`]
    /// ```json
    /// {
    ///     "name_record":{
    ///         "name": "alice",
    ///         "address": "juno12......239qw"
    ///     }
    /// }
    /// ```
    NameRecord {
        /// name 
        name: String, 
        /// and the address that the record belongs
        address: String 
    },
    /// Response for when [QueryMsg::Balance] is invoked
    /// ```json
    /// {
    ///     "balance": {
    ///         "name": "alice",
    ///         "amount": [
    ///             {
    ///                 "amount": "1000",
    ///                 "denom": "token"
    ///             },
    ///         ]
    ///     }
    /// }
    /// ```
    Balance { 
        /// name, whose owners balance is requested
        name: String, 
        /// the balance in various denominations
        amount: Vec<Coin> 
    },

    /// Response for when [QueryMsg::GetSaleFlag] is requested
    /// ```json
    /// {
    ///     "sale_flag": {
    ///         "flag": true
    ///     }
    /// }
    /// ```
    SaleFlag { 
        /// current value of the flag
        flag: bool
    },
}
