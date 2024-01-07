use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::api::AppState;
use crate::controllers::CustomResponse;
use crate::repository::user_repository::User;
use crate::services;

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    email: String,
    password: String,
}

pub async fn login(State(state): State<AppState>, Json(body): Json<LoginBody>) -> Result<Json<User>, (StatusCode, Json<CustomResponse>)> {
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
    Ok(Json(user))
}