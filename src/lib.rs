#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
//! # OmniFlix NameService
//!

///  ## Messages
pub mod msg;
#[doc(inline)]
pub use msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};

/// ## States
pub mod state;
#[doc(inline)]
pub use state::Config;

/// ## Contract Entry Points
pub mod contract;
#[doc(inline)]
pub use crate::contract::{execute, instantiate, query};
/// Contract Config Info
/// ## Config Constants
pub mod config;
#[doc(inline)]
pub use config::*;

/// ## Helpers
pub mod helpers;

/// ## Contract Errors
pub mod error;
#[doc(inline)]
pub use crate::error::ContractError;

use cosmwasm_std::Empty;
/// ## CW721 Contract Intigratation Variables
pub use cw721::Cw721Query;
#[doc(hidden)]
pub use cw721_base::{ContractError as Cw721ContractError, InstantiateMsg as Cw721InstantiateMsg};

/// Cw721Contract allows metadata to be stored as an Extension.
/// As there is no additional metadata that needs to be stored, it is Generic [`Empty`](cosmwasm_std::Empty)
pub type Extension = Option<Empty>;
use cw721_base::MintMsg;

/// cw721 base provides MetaData extension for MintMsg as well
pub type Cw721MintMsg = MintMsg<Extension>;
/// cw721 contract initialised using generics
pub type Cw721Contract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
/// cw721 ExecuteMsg with generic over [`Extension`]
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension>;
