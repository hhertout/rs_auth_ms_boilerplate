use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::api::AppState;
use crate::controllers::CustomResponse;
use crate::services;
use crate::services::crypto::generate_jwt;

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    email: String,
    token: String,
}

pub async fn login(State(state): State<AppState>, Json(body): Json<LoginBody>) -> Result<Json<LoginResponse>, (StatusCode, Json<CustomResponse>)> {
    let user = match state.repository.get_user_by_email(&body.email).await {
        Ok(user) => user,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(CustomResponse {
                    message: String::from("Check your information"),
                }),
            ));
        }
    };

    let matching_res = match services::crypto::check_password(&body.password, &user.password) {
        Ok(matching_res) => matching_res,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(CustomResponse {
                    message: String::from("Check your information"),
                }),
            ));
        }
    };
    if !matching_res {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("Check your information"),
            }),
        ));
    }

    let token = match generate_jwt(&user.email) {
        Ok(t) => t,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CustomResponse {
                    message: err.to_string(),
                }),
            ))
        }
    };

    Ok(Json(LoginResponse {
        token,
        email: user.email
    }))
}