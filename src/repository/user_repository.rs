use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use crate::repository::Repository;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    email: String,
    pub(crate) password: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct NewUserResponse {
    id: String,
    email: String,
}

impl Repository {
    pub async fn save_user(&self, user: NewUser) -> Result<NewUserResponse, Error> {
        sqlx::query_as::<_, NewUserResponse>(
            "\
            INSERT INTO public.user (email, password) \
            VALUES ($1, $2)\
            RETURNING id, created_at, updated_at, deleted_at, email ;\
            "
        )
            .bind(user.email)
            .bind(user.password)
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("\
        SELECT id, email, password \
        FROM public.user \
        WHERE email=$1 \
        AND deleted_at IS NULL\
        ")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await
    }
}