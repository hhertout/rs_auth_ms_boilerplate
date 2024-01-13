use std::env;
use auth_api::services::crypto::JwtService;

#[test]
fn test_generate_jwt_success() {
    env::set_var("JWT_SECRET", "azertyuiop");
    let _ = env::var("JWT_SECRET").expect("JWT_SECRET env variable is required");

    let email = "test@example.com";
    let jwt_result = JwtService::generate_jwt(email);
    assert!(jwt_result.is_ok());

    let jwt_token = jwt_result.unwrap();
    assert!(!jwt_token.is_empty());
}

#[test]
#[should_panic(expected = "JWT_SECRET env variable is required")]
fn test_generate_jwt_missing_secret() {
    env::remove_var("JWT_SECRET");
    let email = "test@example.com";
    let _ = JwtService::generate_jwt(email);
}

#[test]
fn test_generate_jwt_empty_email() {
    env::set_var("JWT_SECRET", "valid_secret");
    let empty_email = "";
    let jwt_result = JwtService::generate_jwt(empty_email);
    assert!(jwt_result.is_err());
}

#[test]
fn test_verify_jwt_success() {
    env::set_var("JWT_SECRET", "valid_secret");
    let _ = env::var("JWT_SECRET").expect("JWT_SECRET env variable is required");

    let email = "test@example.com";
    let valid_token = JwtService::generate_jwt(email);
    assert!(valid_token.is_ok());

    let result = JwtService::verify_jwt(valid_token.unwrap().as_str());
    assert!(result.is_ok());

    let claims = result.unwrap();
    assert!(!claims.sub.is_empty());
}

#[test]
fn test_verify_jwt_empty_token() {
    env::set_var("JWT_SECRET", "valid_secret");
    let empty_token = "";
    let result = JwtService::verify_jwt(empty_token);
    assert!(result.is_err());
}

#[test]
fn test_verify_jwt_invalid_secret() {
    env::set_var("JWT_SECRET", "valid_secret");
    let email = "test@example.com";
    let valid_token = JwtService::generate_jwt(email);
    assert!(valid_token.is_ok());

    env::set_var("JWT_SECRET", "invalid_secret");
    let result = JwtService::verify_jwt(valid_token.unwrap().as_str());
    assert!(result.is_err());
}