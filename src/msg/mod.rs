//! It contains all the definitions for various payloads.
mod execute;
mod instantiate;
mod query;

#[doc(inline)]
pub use execute::ExecuteMsg;
#[doc(inline)]
pub use instantiate::InstantiateMsg;
#[doc(inline)]
pub use query::{QueryMsg, QueryResponse};
