use crate::config::{MAX_NAME_LENGTH, MIN_NAME_LENGTH, SUFFIX};
use crate::error::ContractError;
use crate::Cw721Contract;
use crate::Cw721Query;
use cosmwasm_std::{Addr, DepsMut, Env};

// Sanitize name
pub fn sanitize_name(name: String) -> String {
    if name.ends_with(&SUFFIX) {
        name.strip_suffix(&SUFFIX)
            .unwrap()
            .to_string()
            .to_lowercase()
    } else {
        name.to_lowercase()
    }
}

// let's not import a regexp library and just do these checks by hand
fn is_invalid_char(c: char) -> bool {
    let is_valid = c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '_');
    !is_valid
}

/// validate_name returns an error if the name is invalid
/// (we require 3-64 lowercase ascii letters, numbers, or . - _)
pub fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if (name.len() as u64) < MIN_NAME_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH,
        })
    } else if (name.len() as u64) > MAX_NAME_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_NAME_LENGTH,
        })
    } else {
        match name.find(is_invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}

pub fn verified_name_owner(deps: &DepsMut, env: Env, name: String) -> Result<Addr, ContractError> {
    let owner = match Cw721Contract::default().owner_of(deps.as_ref(), env, name.clone(), false) {
        Ok(res) => res.owner,
        Err(_) => return Err(ContractError::UnregisteredName { name }),
    };

    match deps.api.addr_validate(&owner) {
        Ok(owner) => Ok(owner),
        Err(e) => Err(ContractError::Std(e)),
    }
}
