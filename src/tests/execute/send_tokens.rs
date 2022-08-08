#[test]
fn send_tokens_works_with_sufficient_fund() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &coins(10, "ujunox"));
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: coins(10, "ujunox"),
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => {},
        Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
    };
}

#[test]
fn send_tokens_works_with_extra_fund() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &coins(15, "ujunox"));
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: coins(10, "ujunox"),
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => {},
        Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
    };
}

#[test]
fn send_tokens_fails_with_insufficient_fund() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &coins(8, "ujunox"));
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: coins(10, "ujunox"),
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(r) => panic!("Expected an error: {:#?}", r),
        Err(ContractError::InsufficientFundsSent {  }) => {},
        Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
    };
}

#[test]
fn send_tokens_fails_with_insufficient_funds() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &[coin(8, "ujunox"), coin(8, "ucosm")]);
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(r) => panic!("Expected an error: {:#?}", r),
        Err(ContractError::InsufficientFundsSent {  }) => {},
        Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
    };
}

#[test]
fn send_tokens_fails_with_insufficient_funds2() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &[coin(8, "ujunox"), coin(10, "ucosm")]);
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(r) => panic!("Expected an error: {:#?}", r),
        Err(ContractError::InsufficientFundsSent {  }) => {},
        Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
    };
}

#[test]
fn send_tokens_fails_with_insufficient_funds3() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &[coin(10, "ujunox"), coin(8, "ucosm")]);
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(r) => panic!("Expected an error: {:#?}", r),
        Err(ContractError::InsufficientFundsSent {  }) => {},
        Err(e) => panic!("Unexpected Error Occured -> {:#?}", e)
    };
}

#[test]
fn send_tokens_works_with_sufficient_funds() {
    let mut deps = mock_dependencies();
    let required_coin = coin(2, "token");
    let required_coins = vec![required_coin.clone()];
    mock_init_with_price(deps.as_mut(), required_coin.clone(), required_coin.clone());
    mock_register_name(deps.as_mut(), "alice_key", "alice", &required_coins);
    mock_register_name(deps.as_mut(), "bob_key", "bob", &required_coins);
    let info = mock_info("alice_key", &[coin(10, "ujunox"), coin(10, "ucosm")]);
    let msg = ExecuteMsg::SendTokens {
        name: "bob".to_string(),
        amount: vec![coin(10, "ujunox"), coin(10, "ucosm")],
    };

    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => {},
        Err(e) => panic!("Unxpected an error: {:#?}", e),
    };
}