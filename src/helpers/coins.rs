use crate::error::ContractError;
use cosmwasm_std::Coin;

/// A function to check if sufficient coin for a given action is sent.
/// 
/// params:
/// sent: The sent coin, usually `info.funds`
/// required: Required Coin is set by the contract user during instantiation.
///         It is stored in `Config { purchase_price || transfer_price }`
/// 
/// *note: This function is used for registration and transfer endpoints only.
/// For SendTokens endpoint, [`assert_sent_sufficient_coins`](fn.assert_sent_sufficient_coins.html) is used.* 
pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_sufficient_funds = sent.iter().any(|coin| {
                // check if a given sent coin matches denom
                // and has sufficient amount
                coin.denom == required_coin.denom && coin.amount.u128() >= required_amount
            });

            if sent_sufficient_funds {
                return Ok(());
            } else {
                return Err(ContractError::InsufficientFundsSent {});
            }
        }
    }
    Ok(())
}

/// To check if the sender sent sufficient coins to the contract, for the SendTokens endpoint
/// intended to be sent to the owner of a name. Unlike `assert_sent_sufficient_coin`, this can
/// be used to check for presense of multiple coins.
/// 
/// ```notest
/// assert_sent_sufficient_coins(&info.funds, &msg.amount);
/// ```
pub fn assert_sent_sufficient_coins(sent: &[Coin], required: &[Coin]) -> Result<(), ContractError> {
    if required.iter().all(|required_coin| {
        let required_amount = required_coin.amount.u128();
        if required_amount != 0 {
            sent.iter().any(|sent_coin| {
                sent_coin.denom == required_coin.denom && sent_coin.amount.u128() >= required_amount
            })
        } else {
            true
        }
    }) {
        Ok(())
    } else {
        Err(ContractError::InsufficientFundsSent {})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{coin, coins};

    #[test]
    fn assert_sent_sufficient_coin_works() {
        match assert_sent_sufficient_coin(&[], Some(coin(0, "token"))) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        match assert_sent_sufficient_coin(&[], Some(coin(5, "token"))) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        match assert_sent_sufficient_coin(&coins(10, "smokin"), Some(coin(5, "token"))) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        match assert_sent_sufficient_coin(&coins(10, "token"), Some(coin(5, "token"))) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let sent_coins = vec![coin(2, "smokin"), coin(5, "token"), coin(1, "earth")];
        match assert_sent_sufficient_coin(&sent_coins, Some(coin(5, "token"))) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };
    }

    #[test]
    fn assert_sent_sufficient_coins_work() {
        match assert_sent_sufficient_coins(&[], &[coin(0, "ujunox")]) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(&[coin(10, "ujunox")], &[coin(10, "ujunox")]) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(&[coin(10, "ujunox")], &[coin(8, "ujunox")]) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(&[coin(8, "ujunox")], &[coin(10, "ujunox")]) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(
            &[coin(10, "ujunox"), coin(10, "ucosm")],
            &[coin(10, "ucosm"), coin(10, "ujunox")],
        ) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(
            &[coin(10, "ujunox"), coin(8, "ucosm")],
            &[coin(10, "ucosm"), coin(10, "ujunox")],
        ) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(
            &[coin(8, "ujunox"), coin(8, "ucosm")],
            &[coin(10, "ucosm"), coin(10, "ujunox")],
        ) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        match assert_sent_sufficient_coins(
            &[coin(8, "ujunox"), coin(10, "ucosm")],
            &[coin(10, "ucosm"), coin(10, "ujunox")],
        ) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSent {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
