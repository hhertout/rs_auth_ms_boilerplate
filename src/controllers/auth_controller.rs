use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::SET_COOKIE;
use axum::Json;
use axum::response::{IntoResponse, Response};
use cookie::SameSite;
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

pub async fn login(State(state): State<AppState>, Json(body): Json<LoginBody>) -> Result<Response, (StatusCode, Json<CustomResponse>)> {
    let user = match state.repository.find_user_by_email(&body.email).await {
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
            ));
        }
    };

    let cookie = cookie::Cookie::build(("Authorization", token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();

    let response = (
        [(SET_COOKIE, cookie.to_string())], // headers
        Json(CustomResponse { message: String::from("Successfully logged in !") }) // body
    ).into_response();

    Ok(response)
}

#[derive(Serialize, Deserialize)]
pub struct CheckTokenBody {
    token: String,
}

pub async fn check_token(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<CustomResponse>, (StatusCode, Json<CustomResponse>)> {
    let token = match headers.get("Authorization") {
        Some(t) => t.to_str().unwrap(),
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(CustomResponse {
                    message: String::from("Unauthorized"),
                }),
            ));
        }
    };
    let claims = match services::crypto::verify_jwt(token.to_owned()) {
        Ok(c) => c,
        Err(err) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(CustomResponse {
                    message: err.to_string(),
                }),
            ));
        }
    };

    match state.repository.find_user_by_email(&claims.sub).await {
        Ok(_) => Ok(Json(CustomResponse {
            message: String::from("JWT is valid")
        })),
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(CustomResponse {
                message: String::from("Unauthorized"),
            }),
        ))
    }
}