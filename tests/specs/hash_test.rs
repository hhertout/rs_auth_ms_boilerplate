use auth_api::services::crypto::{check_password, hash_password};

#[test]
fn test_hash_password_success() {
    let password = "password123";
    let hashed_password_result = hash_password(password);
    assert!(hashed_password_result.is_ok());

    let hashed_password = hashed_password_result.unwrap();
    assert_ne!(hashed_password, password);
}

#[test]
fn test_hash_password_empty() {
    let empty_password = "";
    let empty_password_result = hash_password(empty_password);
    assert!(empty_password_result.is_err());
}

#[test]
fn test_hash_password_random() {
    let password = "a9#B!2cD";
    let hashed_password_result = hash_password(password);
    assert!(hashed_password_result.is_ok());
}

#[test]
fn test_check_password_matching() {
    let password = "password123";
    let hashed_password = hash_password(password).unwrap();

    let password_match_result = check_password(password, &hashed_password);
    assert!(password_match_result.is_ok());
    assert!(password_match_result.unwrap());
}

#[test]
fn test_check_password_not_matching() {
    let password = "password123";
    let incorrect_password = "incorrect_password";
    let hashed_password = hash_password(password).unwrap();

    let password_match_result = check_password(incorrect_password, &hashed_password);
    assert!(password_match_result.is_ok());
    assert!(!password_match_result.unwrap());
}

#[test]
fn test_check_password_invalid_hash() {
    let invalid_hash = "invalid_hash";
    let password = "password123";

    let password_match_result = check_password(password, invalid_hash);
    assert!(password_match_result.is_err());
}

#[test]
fn test_check_password_empty() {
    let empty_password = "";
    let hashed_password = hash_password("password123").unwrap();

    let password_match_result = check_password(empty_password, &hashed_password);
    assert!(password_match_result.is_err());
}