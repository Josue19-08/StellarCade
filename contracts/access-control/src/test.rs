#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env, IntoVal};

#[test]
fn test_init_and_has_role() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(AccessControl, ());
    let client = AccessControlClient::new(&env, &contract_id);

    client.init(&admin);

    assert_eq!(client.get_admin(), admin);
    assert!(client.has_role(&ADMIN, &admin));
}

#[test]
fn test_grant_and_revoke_role() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let contract_id = env.register(AccessControl, ());
    let client = AccessControlClient::new(&env, &contract_id);

    client.init(&admin);

    // Grant OPERATOR role
    client.grant_role(&OPERATOR, &user);
    assert!(client.has_role(&OPERATOR, &user));

    // Check state
    assert!(client.has_role(&OPERATOR, &user));

    // Revoke role
    client.revoke_role(&OPERATOR, &user);
    assert!(!client.has_role(&OPERATOR, &user));
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_already_initialized() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = env.register(AccessControl, ());
    let client = AccessControlClient::new(&env, &contract_id);

    client.init(&admin);
    client.init(&admin);
}

#[test]
fn test_non_admin_cannot_grant_role() {
    let env = Env::default();
    
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    let contract_id = env.register(AccessControl, ());
    let client = AccessControlClient::new(&env, &contract_id);

    client.init(&admin);

    env.mock_auths(&[
        soroban_sdk::testutils::MockAuth {
            address: &non_admin,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: &contract_id,
                fn_name: "grant_role",
                args: vec![&env, OPERATOR.into_val(&env), user.into_val(&env)],
                sub_invokes: &[],
            },
        },
    ]);

    let result = client.try_grant_role(&OPERATOR, &user);
    assert!(result.is_err());
}

#[test]
fn test_duplicate_grant_revoke() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let contract_id = env.register(AccessControl, ());
    let client = AccessControlClient::new(&env, &contract_id);

    client.init(&admin);

    // Grant twice
    client.grant_role(&OPERATOR, &user);
    assert!(client.has_role(&OPERATOR, &user));
    client.grant_role(&OPERATOR, &user);
    assert!(client.has_role(&OPERATOR, &user));

    // Revoke twice
    client.revoke_role(&OPERATOR, &user);
    assert!(!client.has_role(&OPERATOR, &user));
    client.revoke_role(&OPERATOR, &user);
    assert!(!client.has_role(&OPERATOR, &user));
}
