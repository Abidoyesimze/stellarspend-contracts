#![cfg(test)]

use soroban_sdk::{Env};

use crate::LMSContract;

#[test]
fn test_initialize() {
    let env = Env::default();

    let result = LMSContract::initialize();

    assert!(result);
}