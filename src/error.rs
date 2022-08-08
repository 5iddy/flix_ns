//! # Error Variants for the OmniFlix NameService Contract

use cosmwasm_std::StdError;
use thiserror::Error;
use cw721_base::ContractError as Cw721ContractError;

/// ## Contract Errors
/// The following are the errors that might raise while handling
/// various entry points.
#[derive(Error, Debug)]
pub enum ContractError {
    /// For converting error from [`cosmwasm_std::StdError`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/enum.StdError.html "Documentation of CosmWasm StdError Enum")
    /// Into Name Service Contract Error Variant
    #[error("{0}")]
    Std(#[from] StdError),

    /// For convering Cw721ContractError variants to NameSerivce ContractError variants
    #[error("{0}")]
    Cw721ContractError(#[from] Cw721ContractError),

    /// Raised when a wallet is not permitted to excute various actions
    /// due to lack of right permissions
    #[error("Unauthorized")]
    Unauthorized {},

    /// Raised when a wallet tries to execute an action with insufficient funds.
    /// For example, they might not have sent suffient coin for transfer or register of a name/NFT.
    #[error("Insufficient funds sent")]
    InsufficientFundsSent {},

    /// Raised when an action encounters access to unregistered or unfound names.
    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },

    /// Raised when an attempt to register a previously taken name is encountered.
    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    /// Raised when the name for registration is too short or less than `crate::config::MIN_NAME_LENGTH`
    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    /// Raised when the name for registration is longer than `crate::config:MAX_NAME_LENGTH`
    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    /// Raised when an invalid character is found in name during its validation
    /// Lowercase AlphaNumeric Characters and an underscore is allowed in the name.
    /// Refer to `./src/helpers/name.rs -> validate_name or is_invalid_char`
    #[error("Invalid character (char {c}) -> Name can only lower case alphabets, numbers, '.', '-' and '_'")]
    InvalidCharacter { c: char },

    /// Raised when an attempt to transfer a name to a non-existent account is encountered.
    #[error("Cannot send tokens to self. {to_address}")]
    InvalidToAddress { to_address: String },
}
