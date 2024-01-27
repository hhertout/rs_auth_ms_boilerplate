use std::thread::sleep;
use std::time::Duration;
use auth_api::services::crypto::CSRFTokenService;

#[test]
fn test_generate_csrf_token() {
    let result = CSRFTokenService::generate_csrf_token();

    assert!(result.is_ok());
    let token = result.unwrap();
    assert!(!token.is_empty());
}

#[test]
fn test_generate_csrf_token_with_different_timestamps() {
    let result1 = CSRFTokenService::generate_csrf_token();
    sleep(Duration::from_millis(800));
    let result2 = CSRFTokenService::generate_csrf_token();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    let token1 = result1.unwrap();
    let token2 = result2.unwrap();
    assert_ne!(token1, token2);
}

#[test]
fn test_generate_csrf_token_consistency() {
    let result1 = CSRFTokenService::generate_csrf_token();
    let result2 = CSRFTokenService::generate_csrf_token();

    assert_eq!(result1, result2);
}