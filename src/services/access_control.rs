use std::io::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use crate::database::{DatabaseService, Database};

pub trait GrantAccess {
    fn from_role(role: Vec<String>, granted_roles: Vec<String>) -> Authorization;
    async fn with_email(&self, email: &str, granted_roles: Vec<String>) -> Authorization;
}

pub enum Authorization {
    Authorized,
    Unauthorized(Error),
}

#[derive(FromRow, Serialize, Deserialize)]
struct User {
    role: Option<Vec<String>>,
}

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
    fn from_role(roles: Vec<String>, granted_roles: Vec<String>) -> Authorization {
        for role in roles {
            if granted_roles.contains(&role) {
                return Authorization::Authorized;
            }
        }

        Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
    }

    async fn with_email(&self, email: &str, granted_roles: Vec<String>) -> Authorization {
        let res = sqlx::query_as::<_, User>("SELECT role FROM public.user WHERE email=$1")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await;

        let user_found = match res {
            Ok(user) if user.role.is_some() => user,
            _ => return Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "User not found"))
        };

        for role in user_found.role.unwrap() {
            if granted_roles.contains(&role) {
                return Authorization::Authorized;
            }
        }

        Authorization::Unauthorized(Error::new(ErrorKind::InvalidData, "Unauthorized"))
    }
}