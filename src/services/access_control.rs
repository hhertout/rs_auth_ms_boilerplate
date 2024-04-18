use std::io::{Error, ErrorKind};
use std::str::FromStr;
use axum::http::HeaderMap;
use cookie::Cookie;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use crate::config::roles::Role;
use crate::controllers::auth_controller::extract_auth_cookie;
use crate::database::{DatabaseService, Database};
use crate::services::crypto::JwtService;

#[allow(async_fn_in_trait)]
pub trait GrantAccess {
    fn from_role(role: Vec<Role>, granted_roles: Vec<Role>) -> Authorization;
    async fn with_email(&self, email: &str, granted_roles: Vec<Role>) -> Authorization;
    async fn with_cookie(&self, cookie_header: HeaderMap, granted_roles: Vec<Role>) -> Authorization;
}

pub enum Authorization {
    Authorized,
    Unauthorized(Error),
}

#[derive(FromRow, Serialize, Deserialize)]
struct User {
    role: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct AccessControl {
    db_pool: Pool<Postgres>,
}

impl AccessControl {
    pub async fn new() -> AccessControl {
        let pool = DatabaseService::new().database_connection().await;
        AccessControl {
            db_pool: pool
        }
    }
}

impl GrantAccess for AccessControl {
    fn from_role(roles: Vec<Role>, granted_roles: Vec<Role>) -> Authorization {
        for role in roles {
            if granted_roles.contains(&role) {
                return Authorization::Authorized;
            }
        }
        Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
    }

    async fn with_email(&self, email: &str, granted_roles: Vec<Role>) -> Authorization {
        let res = sqlx::query_as::<_, User>("SELECT role FROM public.user WHERE email=$1")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await;

        let user_found = match res {
            Ok(user) if user.role.is_some() => user,
            _ => return Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "User not found"))
        };

        for role in user_found.role.unwrap() {
            match Role::from_str(&role) {
                Ok(role) => {
                    if granted_roles.contains(&role) {
                        return Authorization::Authorized;
                    }
                }
                Err(_) => continue
            }
        }

        Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
    }

    async fn with_cookie(&self, cookie_header: HeaderMap, granted_roles: Vec<Role>) -> Authorization {
        let cookie = match extract_auth_cookie(cookie_header) {
            Ok(cookie) => cookie,
            Err(_) => return Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
        };
        let token = match Cookie::parse(cookie) {
            Ok(cookie) => cookie,
            Err(_) => return Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
        };

        let claims = match JwtService::verify_jwt(token.value()) {
            Ok(claims) => claims,
            Err(_) => return Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
        };
        self.with_email(&claims.sub, granted_roles).await
    }
}