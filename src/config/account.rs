use crate::config::roles::Role;
use crate::database::{DatabaseService, Database};

pub async fn create_super_admin_account() {
    let pool = DatabaseService::new().database_connection().await;

    let email = std::env::var("SUPER_ADMIN_EMAIL")
        .expect("SUPER_ADMIN_EMAIL must be set");
    let password = std::env::var("SUPER_ADMIN_PASSWORD")
        .expect("SUPER_ADMIN_PASSWORD must be set");

    let _ = sqlx::query("INSERT INTO public.user(email, password, role) VALUES ($1, $2, $3) ON CONFLICT(email) DO NOTHING")
        .bind(email)
        .bind(password)
        .bind(vec![Role::SUPER_ADMIN.to_str()])
        .execute(&pool)
        .await;
}