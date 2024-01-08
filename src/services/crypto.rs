use std::env;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Error, SaltString};
use chrono::Utc;
use cookie::time::format_description::well_known::iso8601::FormattedComponents::Date;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm, decode, Validation, decode_header, DecodingKey};
use serde::{Deserialize, Serialize};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);

    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn check_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub fn generate_jwt(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| panic!("JWT_SECRET env variable is required"));
    let my_claims = Claims { sub: email.to_owned(), exp: (Utc::now().timestamp() + (3600 * 24 * 20)) as u64 };

    let header = Header {
        kid: Some("signing_key".to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };

    encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn verify_jwt(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| panic!("JWT_SECRET env variable is required"));

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ).map(|data| data.claims)
}