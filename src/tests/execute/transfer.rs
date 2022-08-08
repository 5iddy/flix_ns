use crate::tests::{
    mock_dependencies
};

#[test]
fn transfer_works() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    mock_alice_registers_name(deps.as_mut(), &[]);

    // alice can transfer her name successfully to bob
    let info = mock_info("alice_key", &[]);
    let msg = ExecuteMsg::TransferName {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let _res = execute(deps.as_mut(), mock_env(), info, msg)
        .expect("contract successfully handles Transfer message");
    // querying for name resolves to correct address (bob_key)
    assert_name_owner(deps.as_ref(), "alice", "bob_key");
}

#[test]
fn transfer_works_with_fees() {
    let mut deps = mock_dependencies();
    mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
    mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

    // alice can transfer her name successfully to bob
    let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
    let msg = ExecuteMsg::TransferName {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let _res = execute(deps.as_mut(), mock_env(), info, msg)
        .expect("contract successfully handles Transfer message");
    // querying for name resolves to correct address (bob_key)
    assert_name_owner(deps.as_ref(), "alice", "bob_key");
}

#[test]
    fn fails_on_transfer_non_existent() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // alice can transfer her name successfully to bob
        let info = mock_info("frank_key", &coins(2, "token"));
        let msg = ExecuteMsg::TransferName {
            name: "alice42".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameNotExists { name }) => assert_eq!(name, "alice42"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        // querying for name resolves to correct address (alice_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

#[test]
fn fails_on_transfer_from_nonowner() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    mock_alice_registers_name(deps.as_mut(), &[]);

    // alice can transfer her name successfully to bob
    let info = mock_info("frank_key", &coins(2, "token"));
    let msg = ExecuteMsg::TransferName {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Unauthorized { .. }) => {}
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // querying for name resolves to correct address (alice_key)
    assert_name_owner(deps.as_ref(), "alice", "alice_key");
}

#[test]
fn fails_on_transfer_insufficient_fees() {
    let mut deps = mock_dependencies();
    mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(5, "token"));
    mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

    // alice can transfer her name successfully to bob
    let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
    let msg = ExecuteMsg::TransferName {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Ok(_) => panic!("register call should fail with insufficient fees"),
        Err(ContractError::InsufficientFundsSent {}) => {}
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // querying for name resolves to correct address (bob_key)
    assert_name_owner(deps.as_ref(), "alice", "alice_key");
}