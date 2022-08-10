use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSent {},

    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    #[error("Invalid character (char {c}) -> Name can only lower case alphabets, numbers, '.', '-' and '_'")]
    InvalidCharacter { c: char },

    #[error("Cannot send tokens to self. {to_address}")]
    InvalidToAddress { to_address: String }
}
