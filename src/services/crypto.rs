use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Local, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::digest::block_buffer::Error;
use sha2::Digest;
use sha2::Sha256;
use std::env;

pub struct HashService;
pub struct JwtService;
pub struct CSRFTokenService;

impl Hash for HashService {}
impl Jwt for JwtService {}

pub trait Hash {
    fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        if password.is_empty() {
            return Err(argon2::password_hash::Error::PhcStringField);
        }

        let salt = SaltString::generate(OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    fn check_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        if password.is_empty() || hash.is_empty() {
            return Err(argon2::password_hash::Error::PhcStringField);
        }

        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub trait Jwt {
    fn generate_jwt(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        if email.is_empty() {
            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidIssuer));
        }

        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET env variable is required"));
        let my_claims = Claims {
            sub: email.to_owned(),
            exp: (Utc::now().timestamp() + (3600 * 24 * 20)) as u64,
        };

        /* let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        }; */

        encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        if token.is_empty() {
            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken));
        }

        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET env variable is required"));

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
}

impl CSRFTokenService {
    pub fn generate_csrf_token() -> Result<String, Error> {
        let secret = std::env::var("CSRF_SECRET").map_err(|_| Error)?;
        let ts: String = format!("{}{}", Local::now().timestamp(), secret);
        let mut hasher = Sha256::new();
        hasher.update(ts.as_bytes());
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }
}
