#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Error, InvokeError};

fn assert_contract_error<T>(result: Result<T, Result<Error, InvokeError>>, expected: AssetControlError) {
    match result {
        Err(Ok(error)) => assert_eq!(error, Error::from_contract_error(expected as u32)),
        other => panic!("expected contract error {:?}, got {:?}", expected, other),
    }
}

fn create_contract() -> (Env, Address, AssetControlContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(AssetControlContract, ());
    let client = AssetControlContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, admin, client)
}

#[test]
fn test_blacklist_flow_allows_add_and_remove() {
    let (env, admin, client) = create_contract();
    let asset = Address::generate(&env);

    assert!(!client.is_blacklisted(&asset));

    client.add_to_blacklist(&asset);
    assert!(client.is_blacklisted(&asset));

    client.remove_from_blacklist(&asset);
    assert!(!client.is_blacklisted(&asset));
}

#[test]
fn test_check_asset_panics_for_blacklisted_asset() {
    let (env, admin, client) = create_contract();
    let asset = Address::generate(&env);

    client.add_to_blacklist(&asset);

    let result = client.try_check_asset(&asset);
    assert_contract_error(result, AssetControlError::Unauthorized);
}

#[test]
fn test_check_asset_does_not_panic_for_non_blacklisted_asset() {
    let (env, _, client) = create_contract();
    let asset = Address::generate(&env);

    client.check_asset(&asset);
}

#[test]
fn test_unauthorized_caller_cannot_add_to_blacklist() {
    let (env, _, client) = create_contract();
    let unauthorized = Address::generate(&env);
    let asset = Address::generate(&env);

    let result = client.try_add_to_blacklist(&asset);
    assert_contract_error(result, AssetControlError::Unauthorized);
}
