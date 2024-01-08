use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::api::AppState;
use crate::controllers::CustomResponse;
use crate::repository::user_repository::{NewUser, NewUserResponse};
use crate::services::crypto::hash_password;

pub async fn save_user(State(state): State<AppState>, Json(mut body): Json<NewUser>) -> Result<Json<NewUserResponse>, (StatusCode, Json<CustomResponse>)> {
    body.password = match hash_password(&body.password) {
        Ok(h) => h,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CustomResponse {
                    message: err.to_string(),
                })
            ));
        }
    };

    let res = state.repository.save_user(body).await;
    match res {
        Ok(new_user) => Ok(Json(new_user)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))
    }
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
    let res = state.repository.find_user_by_email(&body.email).await;
    match res {
        Ok(user) => Ok(Json(UserResponse {
            id: user.id,
            email: user.email,
        })),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            })
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    email: String,
    password: String,
}

pub async fn update_password(State(state): State<AppState>, Json(body): Json<ChangePasswordRequest>) -> Result<Json<CustomResponse>, (StatusCode, Json<CustomResponse>)> {
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
    let hash = match hash_password(&body.password) {
        Ok(h) => h,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CustomResponse {
                    message: err.to_string(),
                })
            ));
        }
    };

    match state.repository.update_user_password(&user.id, &hash).await {
        Ok(success) => {
            if success {
                Ok(Json(CustomResponse {
                    message: String::from("Password update successfully !")
                }))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(CustomResponse {
                        message: String::from("Failed to update password"),
                    })
                ))
            }
        }
        Err(err) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CustomResponse {
                    message: err.to_string(),
                })
            ))
        }
    }
}