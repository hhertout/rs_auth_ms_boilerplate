use std::env;
use sqlx::{Pool, Postgres};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;

#[derive(Default)]
pub(crate) struct Database;

impl Database {
    pub fn new() -> Database {
        Database
    }

    pub async fn database_connection(&self) -> Pool<Postgres> {
        let db_url = env::var("DB_URL").unwrap_or_else(|_| panic!("DB_URL env variable is not set"));
        PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap_or_else(|_| panic!("Failed to connect to database. {}", db_url))
    }

    #[allow(dead_code)]
    pub async fn migrations_migrate(&self) {
        let _ = &self.create_database().await;
        let pool = &self.database_connection().await;
        sqlx::migrate!()
            .run(pool)
            .await
            .unwrap_or_else(|err| panic!("Migration failed : {:?}", err))
    }

    #[allow(dead_code)]
    async fn create_database(&self) {
        let db_url = env::var("DB_URL").unwrap();
        if !Postgres::database_exists(&db_url).await.unwrap_or(false) {
            match Postgres::create_database(&db_url).await {
                Ok(_) => println!("Database successfully created"),
                Err(e) => println!("Error during the creation of the database. {}", e),
            };
        }
    }
}