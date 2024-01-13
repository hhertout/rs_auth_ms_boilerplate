use auth_api::services::access_control::{AccessControl, Authorization, GrantAccess};


#[test]
pub fn verify_from_role_test() {
    let valid_role = vec![String::from("ROLE_USER")];
    let granted_role = vec![String::from("ROLE_USER")];

    let res = match AccessControl::from_role(valid_role, granted_role.clone()) {
        Authorization::Authorized => true,
        Authorization::Unauthorized(_) => false
    };
    assert!(res);

    let invalid_role = vec![String::from("ROLE_INVALID")];

    let res = match AccessControl::from_role(invalid_role, granted_role) {
        Authorization::Authorized => true,
        Authorization::Unauthorized(_) => false
    };
    assert!(!res);
}