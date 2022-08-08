#[test]
fn returns_empty_on_query_unregistered_name() {
    let mut deps = mock_dependencies();

    mock_init_no_price(deps.as_mut());

    // querying for unregistered name results in NotFound error
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::ResolveRecord {
            name: "alice".to_string(),
        },
    )
    .unwrap();
    let value: ResolveRecordResponse = from_binary(&res).unwrap();
    assert_eq!(None, value.address);
}
