use std::env;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm, decode, DecodingKey, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};

pub struct HashService;
pub struct JwtService;

impl HashService {
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        if password.is_empty() {
            return Err(argon2::password_hash::Error::PhcStringField);
        }

        let salt = SaltString::generate(OsRng);
        let argon2 = Argon2::default();
        Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
    }

    pub fn check_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        if password.is_empty() || hash.is_empty() {
            return Err(argon2::password_hash::Error::PhcStringField);
        }

        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

impl JwtService {
    pub fn generate_jwt(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        if email.is_empty() {
            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidIssuer));
        }

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

    pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        if token.is_empty() {
            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken));
        }

        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET env variable is required"));

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        ).map(|data| data.claims)
    }
}


