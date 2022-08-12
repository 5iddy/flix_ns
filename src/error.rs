//! # Error Variants for the OmniFlix NameService Contract

use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721ContractError;
use thiserror::Error;

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
    /// Also Refer to: [`QueryMsg::Config`](crate::msg::QueryMsg) and [`Config`](crate::Config)
    #[error("Insufficient funds sent")]
    InsufficientFundsSent {},

    /// Raised when an action encounters access to unregistered or unfound names.
    #[error("Name does not exist: {name}")]
    UnregisteredName { 
        /// Name which was responsible for this error
        name: String 
    },

    /// Raised when an attempt to register a previously taken name is encountered.
    #[error("Name has been taken: {name}")]
    NameTaken { 
        /// Name which was responsible for this error
        name: String 
    },

    /// Raised when the name for registration is too short or less than [`MIN_NAME_LENGTH`](crate::config::MIN_NAME_LENGTH)
    #[error("Name too short: {length} but minimum length should be: {min_length})")]
    NameTooShort { 
        /// Length of the current name that was responsible for this error
        length: u64, 
        /// Minimum length allowed for the name
        /// Also Refer to: [`MIN_NAME_LENGTH`](crate::config::MIN_NAME_LENGTH)
        min_length: u64 
    },

    /// Raised when the name for registration is longer than [`MAX_NAME_LENGTH`](crate::config::MAX_NAME_LENGTH)
    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { 
        /// Length of the current name that was responsible for this error
        length: u64, 
        /// Maximum length allowed for the name
        /// Also Refer to: [`MAX_NAME_LENGTH`](crate::config::MAX_NAME_LENGTH)
        max_length: u64 
    },

    /// Raised when an invalid character is found in name during its validation
    /// Lowercase AlphaNumeric Characters and an underscore is allowed in the name.
    /// Refer to `./src/helpers/name.rs -> validate_name or is_invalid_char`
    #[error("Invalid character (char {c}) -> Name can only lower case alphabets, numbers, '.', '-' and '_'")]
    InvalidCharacter {
        /// The first invalid character that is found
        c: char 
    },

    /// Raised when an attempt to transfer a name to a non-existent account is encountered.
    #[error("Cannot send tokens to self. {to_address}")]
    InvalidToAddress {
        /// The invalid address that caused this error
        to_address: String 
    },

    /// When sale_flag is false, the Sale Window is considered to be closed.
    /// No new registrations can be made if the sale_flag is false.
    /// This error is raised when a registration is executed while
    /// the sale_flag is false.
    #[error("Sale Window is now closed. sale flag: {flag}")]
    ClosedSaleWindow { 
        /// the current value of the sale_flag
        flag: bool 
    },
}
