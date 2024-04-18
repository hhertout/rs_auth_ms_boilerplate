use std::str::FromStr;
use auth_api::config::roles::Role;
use auth_api::services::access_control::{AccessControl, Authorization, GrantAccess};

#[test]
#[should_panic]
pub fn verify_from_role_test() {
    let valid_role = vec![Role::USER];
    let granted_role = vec![Role::USER];

    let res = match AccessControl::from_role(valid_role, granted_role.clone()) {
        Authorization::Authorized => true,
        Authorization::Unauthorized(_) => false
    };
    assert!(res);

    let invalid_role = vec![Role::from_str("INVALID_ROLE").unwrap()];

    let res = match AccessControl::from_role(invalid_role, granted_role.clone()) {
        Authorization::Authorized => true,
        Authorization::Unauthorized(_) => false
    };
    assert!(!res);

    let empty_role = vec![];

    let res = match AccessControl::from_role(empty_role, granted_role.clone()) {
        Authorization::Authorized => true,
        Authorization::Unauthorized(_) => false
    };
    assert!(!res);
}