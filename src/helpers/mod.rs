//! Contains various helper functions required for various tasks.

#[doc(hidden)]
mod name;
#[doc(inline)]
pub use name::{sanitize_name, validate_name, verified_name_owner};

#[doc(hidden)]
mod coins;
#[doc(inline)]
pub use coins::{assert_sent_sufficient_coin, assert_sent_sufficient_coins};

#[cfg(test)]
pub mod testing;
