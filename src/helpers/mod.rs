pub mod name;
pub use name::{sanitize_name, validate_name};

pub mod coins;
pub use coins::{assert_sent_sufficient_coin, assert_sent_sufficient_coins};

pub mod testing;