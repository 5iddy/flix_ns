use crate::state::Config;
use cw_storage_plus::Item;

/// The suffix for the name
pub const SUFFIX: &str = ".flix";
/// Minimum length of name in u64 characters
pub const MIN_NAME_LENGTH: u64 = 3;
/// Maximum length of the name in u64 characters
pub const MAX_NAME_LENGTH: u64 = 64;
/// Config state instance where the configuration info is stored.
/// ```notest
/// let config: Config = CONFIG.load(deps.storage)?;
/// ```
pub const CONFIG: Item<Config> = Item::new("config");
