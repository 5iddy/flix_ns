//! # OmniFlix NameService
//!

///  ## Messages
pub mod msg;
pub use msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};

/// ## States
pub mod state;
pub use state::Config;

/// ## Contract Entry Points
pub mod contract;
pub use crate::contract::{execute, instantiate, query};
/// Contract Config Info
/// ## Config Constants
pub mod config;
pub use config::*;

/// ## Helpers
pub mod helpers;

/// ## Contract Errors
pub mod error;
pub use crate::error::ContractError;

use cosmwasm_std::Empty;
/// ## CW721 Contract Intigratation Variables
pub use cw721::Cw721Query;
pub use cw721_base::{ContractError as Cw721ContractError, InstantiateMsg as Cw721InstantiateMsg};
pub type Extension = Option<Empty>;
use cw721_base::MintMsg;
pub type Cw721MintMsg = MintMsg<Extension>;
pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;
