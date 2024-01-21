use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use crate::repository::Repository;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) role: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) role: Vec<String>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct NewUserResponse {
    id: String,
    email: String,
    role: Vec<String>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct UserProgression {
    pub(crate) creation_date: NaiveDate,
    pub(crate) incr_count: i32
}

impl Repository {
    pub async fn save_user(&self, user: NewUser) -> Result<NewUserResponse, Error> {
        sqlx::query_as::<_, NewUserResponse>(
            "\
            INSERT INTO public.user (email, password, role) \
            VALUES ($1, $2, $3)\
            RETURNING id, created_at, updated_at, deleted_at, email, role;\
            "
        )
            .bind(user.email)
            .bind(user.password)
            .bind(user.role)
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("\
        SELECT id, email, password, role \
        FROM public.user \
        WHERE email=$1 \
        AND deleted_at IS NULL\
        ")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn find_banned_user_by_email(&self, email: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("\
        SELECT id, email, password \
        FROM public.user \
        WHERE email=$1 \
        AND deleted_at IS NOT NULL\
        ")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await
    }

    pub async fn update_user_password(&self, user_id: &str, password: &str) -> Result<(), Error> {
        let res = sqlx::query("UPDATE public.user SET password=$1 WHERE id=$2")
            .bind(password)
            .bind(user_id)
            .execute(&self.db_pool)
            .await?;

        self.is_row_affected(res.rows_affected(), 1)
    }

    pub async fn soft_delete_user(&self, id: &str) -> Result<(), Error> {
        let res = sqlx::query("UPDATE public.user SET deleted_at=now() WHERE id=$1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;

        self.is_row_affected(res.rows_affected(), 1)
    }

    pub async fn remove_soft_deletion_user(&self, id: &str) -> Result<(), Error> {
        let res = sqlx::query("UPDATE public.user SET deleted_at=null WHERE id=$1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;

        self.is_row_affected(res.rows_affected(), 1)
    }

    pub async fn hard_delete_user(&self, id: &str) -> Result<(), Error> {
        let res = sqlx::query("DELETE FROM public.user WHERE id=$1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;

        self.is_row_affected(res.rows_affected(), 1)
    }

    pub async fn get_v_user_progression(&self) -> Result<Vec<UserProgression>, Error> {
        sqlx::query_as::<_, UserProgression>("SELECT * FROM v_user_progression")
            .fetch_all(&self.db_pool)
            .await
    }
}