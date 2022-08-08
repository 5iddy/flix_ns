//! # OmniFlix NameService
//!

///  ## Messages
pub mod msg;
pub use msg::{
    InstantiateMsg,
    ExecuteMsg,
    QueryMsg,
    ResolveRecordResponse
};
pub use cw721_base::{
    InstantiateMsg as Cw721InstantiateMsg,
    ContractError as Cw721ContractError
};
pub use cw721::Cw721Query;

/// ## States
pub mod state;
/// Contract Config Info
pub use state::Config;

/// ## Contract Entry Points
pub mod contract;
pub use crate::contract::{
    execute, 
    instantiate, 
    query
};

/// ## Config Constants
pub mod config;
pub use config::*;

/// ## Helpers
pub mod helpers;

/// ## Contract Errors
pub mod error;
pub use crate::error::ContractError;


/// ## CW721 Contract Intigratation Variables
use cosmwasm_std::Empty;

pub type Extension = Option<Empty>;
use cw721_base::MintMsg; pub type Cw721MintMsg = MintMsg<Extension>;
pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;


/// ## Contract Tests
#[cfg(test)]
mod tests;

