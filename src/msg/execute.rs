use cosmwasm_std::Coin;
use cw721::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


/// ExecuteMsg is the main JSON serializable enum.
/// Based on its variant, [`execute`](fn.execute.html) will handle the execution.
/// Its variants are the main entry to various end points.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Register a name for a wallet
    ///  
    /// **Example**:
    ///
    /// ```json
    ///     {
    ///         "register": {
    ///             "name": "alice"
    ///         }
    ///     }
    /// ```
    Register { 
        /// Name to be registered. This name resolves to its owners address.
        /// Name can only contain alphabets, digits and an underscore.
        /// Before the name is register, ".flix" suffix is stripped if it is present, and
        /// it is convereted to lower case.
        name: String 
    },

    /// Transfer the ownership to a different wallet
    ///  
    /// **Example**:
    ///
    /// ```json
    ///     {
    ///         "transfer_name": {
    ///             "name": "alice",
    ///             "to": "juno12......we34ex"
    ///         }
    ///     }
    /// ```
    TransferName { 
        /// A name that is owned by the message sender that needs to be transfered.
        name: String, 
        /// The address of the new owner to whom the aforementioned name is being transfered to.
        to: String 
    },

    /// Send tokens to aa wallet based on their name
    ///  
    /// **Example**:
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
    SendTokens { 
        /// The owner of this name who will receive these tokens.
        name: String,
        /// The amount that the message sender wants to send to the owner of the name.
        /// The same amount (or more) mentioned here must also be sent to the contract.
        amount: Vec<Coin> 
    },

    /// For deleting an NFT or name. The record will be deleted and thus will be
    /// available for registration.  
    ///  
    /// **Example**:
    /// 
    /// ```json
    ///     {
    ///         "burn": {
    ///             "name": "alice"
    ///         }
    ///     }
    /// ```
    Burn { 
        /// Name that needs to be burned or deleted.
        name: String 
    },

    /// A name owner can temporarily or permanantly give permission to spend
    /// or transfer a Name NFT to anyone. Approvals are required if the account user
    /// wanted to put it up for sale on the market. Marketplace Smart Contracts need
    /// to seek approval from the account user to permit/allow the transfer permission.
    /// They need execute this message with spender as the contract address, on the name
    /// request until an expiration. After the approval is expired, the spender won't be
    /// able to transfer or spend a Name NFT.
    /// 
    /// **Example**:
    /// 
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
        /// Spender's wallet address (or contract address) who needs to gain transfer
        /// or spend permission on
        spender: String,
        /// the name
        name: String,
        /// until
        expires: Option<Expiration>,
    },

    /// Remove previously granted Approval
    ///  
    /// **Example**:
    ///
    /// ```json
    ///     {
    ///         "revoke": {
    ///             "name": "alice",
    ///             "spender": "cosmwasm12....2wer34s"
    ///         }
    ///     }
    /// ```
    Revoke { 
        /// The spender's wallet address or contract address
        spender: String, 
        /// the name of the NFT on which the the spender until now posses the
        /// approval or permission to spend or transfer NFT.
        name: String 
    },

    /// The Config's `sale_flag` needs to be true, if the accounts are permitted to
    /// register. If `sale_flag` is false, then Registrations are met with a
    /// ClosedSaleWindow Error. By default, if no sale_flag is mentioned in the Initialise 
    /// message, it will be set to true.
    /// 
    /// This variant is used by admin to set the sale_flag. Only the Contract Admin
    /// can call this end point and successfully change the flag.
    /// 
    /// **Example**:
    ///
    /// ```json
    ///     {
    ///         "set_sale": {
    ///             "flag": true
    ///         }
    ///     }
    /// ```
    SetSale { 
        /// Config's `sale_flag` will be set to whatever this value is.
        flag: bool 
    },

    /// Only admin can call this endpoint. He can only do it once. After the
    /// admin is changed, the current admin will lose his powers over the contract.
    /// Unless he recalls this endpoint and return it to its owner.
    /// 
    /// **Example**:
    ///
    /// ```json
    ///     {
    ///         "change_admin" : {
    ///             "admin": "cosmwasm12....2qwe23"
    ///         }
    ///     }
    /// ```
    ChangeAdmin {
        /// the wallet address of the new admin 
        admin: String 
    },
}
