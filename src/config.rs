use cw_storage_plus::Item;
use crate::state::Config;

pub const SUFFIX: &str = ".flix";
pub const MIN_NAME_LENGTH: u64 = 3;
pub const MAX_NAME_LENGTH: u64 = 64;
pub const CONFIG: Item<Config> = Item::new("config");