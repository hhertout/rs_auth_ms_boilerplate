use std::str::FromStr;
use auth_api::config::roles::Role;

#[test]
fn test_to_str() {
    assert_eq!(Role::SUPER_ADMIN.to_str(), "ROLE_SUPER_ADMIN");
    assert_eq!(Role::ADMIN.to_str(), "ROLE_ADMIN");
    assert_eq!(Role::USER.to_str(), "ROLE_USER");
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", Role::SUPER_ADMIN), "ROLE_SUPER_ADMIN");
    assert_eq!(format!("{}", Role::ADMIN), "ROLE_ADMIN");
    assert_eq!(format!("{}", Role::USER), "ROLE_USER");

    assert_eq!(Role::ADMIN.to_string(), String::from("ROLE_ADMIN"));
    assert_eq!(Role::SUPER_ADMIN.to_string(), String::from("ROLE_SUPER_ADMIN"));
    assert_eq!(Role::USER.to_string(), String::from("ROLE_USER"));
}

#[test]
fn test_from_str_valid() {
    assert_eq!(Role::from_str("ROLE_SUPER_ADMIN").unwrap(), Role::SUPER_ADMIN);
    assert_eq!(Role::from_str("ROLE_ADMIN").unwrap(), Role::ADMIN);
    assert_eq!(Role::from_str("ROLE_USER").unwrap(), Role::USER);
}

#[test]
fn test_from_str_invalid() {
    assert!(Role::from_str("INVALID_ROLE").is_err());
}