use sqlx::{Error, Pool, Postgres};
use crate::database::{Database, DatabaseService};

pub mod user_repository;

#[derive(Clone)]
pub struct Repository {
    db_pool: Pool<Postgres>
}

impl Repository {
    pub async fn new() -> Repository {
        let db = DatabaseService::new();
        Repository {
            db_pool: db.database_connection().await
        }
    }

    fn is_row_affected(&self, row_nb: u64, expected: u64) -> Result<(), Error> {
        if row_nb == expected {
            Ok(())
        } else {
            Err(Error::RowNotFound)
        }
    }
}