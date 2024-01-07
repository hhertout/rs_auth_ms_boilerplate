use sqlx::{Pool, Postgres};
use crate::database::Database;

pub mod user_repository;

#[derive(Clone)]
pub struct Repository {
    db_pool: Pool<Postgres>
}

impl Repository {
    pub async fn new() -> Repository {
        let db = Database::new();
        Repository {
            db_pool: db.database_connection().await
        }
    }
}