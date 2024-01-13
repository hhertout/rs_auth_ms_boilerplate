use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::api::AppState;
use crate::controllers::CustomResponse;
use crate::repository::user_repository::{NewUser, NewUserResponse};
use crate::services::crypto::HashService;

#[derive(Serialize, Deserialize)]
pub struct NewUserBody {
    email: String,
    password: String
}

pub async fn save_user(State(state): State<AppState>, Json(body): Json<NewUserBody>) -> Result<Json<NewUserResponse>, (StatusCode, Json<CustomResponse>)> {
    let hash = HashService::hash_password(&body.password)
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    let user  = NewUser {
        email: body.email,
        password: hash,
        role: vec![String::from("ROLE_USER")]
    };

    let new_user = state.repository
        .save_user(user)
        .await
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    Ok(Json(new_user))
}

#[derive(Serialize, Deserialize)]
pub struct GetUserEmailBody {
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    id: String,
    email: String,
}

pub async fn get_user_by_email(State(state): State<AppState>, Json(body): Json<GetUserEmailBody>) -> Result<Json<UserResponse>, (StatusCode, Json<CustomResponse>)> {
    let user = state.repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    email: String,
    password: String,
}

pub async fn update_password(State(state): State<AppState>, Json(body): Json<ChangePasswordRequest>) -> Result<Json<CustomResponse>, (StatusCode, Json<CustomResponse>)> {
    let user = state.repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        ))?;

    let hash = HashService::hash_password(&body.password)
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    state.repository
        .update_user_password(&user.id, &hash)
        .await
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    Ok(Json(CustomResponse {
        message: String::from("Password update successfully !")
    }))
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUserRequest {
    email: String,
}

pub async fn soft_delete_user(State(state): State<AppState>, Json(body): Json<DeleteUserRequest>) -> Result<Json<CustomResponse>, (StatusCode, Json<CustomResponse>)> {
    let user = match state.repository.find_user_by_email(&body.email).await {
        Ok(u) => u,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(CustomResponse {
                    message: String::from("This user doesn't exist"),
                })
            ));
        }
    };

    match state.repository.soft_delete_user(&user.id).await {
        Ok(_) => Ok(Json(CustomResponse {
            message: String::from("User deleted successfully !")
        })),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))
    }
}

pub async fn remove_soft_deletion_user(State(state): State<AppState>, Json(body): Json<DeleteUserRequest>) -> Result<Json<CustomResponse>, (StatusCode, Json<CustomResponse>)> {
    let user = state.repository
        .find_banned_user_by_email(&body.email)
        .await
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        ))?;

    state.repository
        .remove_soft_deletion_user(&user.id)
        .await
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    Ok(Json(CustomResponse {
        message: String::from("User is now accessible !")
    }))
}

pub async fn hard_delete_user(State(state): State<AppState>, Json(body): Json<DeleteUserRequest>) -> Result<Json<CustomResponse>, (StatusCode, Json<CustomResponse>)> {
    let user = state.repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        ))?;

    state.repository
        .hard_delete_user(&user.id)
        .await
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    Ok(Json(CustomResponse {
        message: String::from("User deleted successfully !")
    }))
}

pub async fn get_user_progression(State(state): State<AppState>) -> Result<Response, (StatusCode, Json<CustomResponse>)> {
    let res = state.repository
        .get_v_user_progression()
        .await
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))?;

    Ok(Json(res).into_response())
}
