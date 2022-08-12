#[doc(hidden)]
pub mod execute;
#[doc(hidden)]
pub mod instantiate;
#[doc(hidden)]
pub mod query;

#[doc(inline)]
pub use execute::execute;
#[doc(inline)]
pub use instantiate::instantiate;
#[doc(inline)]
pub use query::query;
