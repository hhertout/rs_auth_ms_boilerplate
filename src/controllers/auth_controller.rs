use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::SET_COOKIE;
use axum::{Json};
use axum::response::{IntoResponse, Response};
use cookie::{Expiration, SameSite};
use cookie::time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use crate::api::AppState;
use crate::controllers::CustomResponse;
use crate::services::crypto::{HashService, JwtService};

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    email: String,
    password: String,
}

pub async fn login(State(state): State<AppState>, Json(body): Json<LoginBody>) -> Result<Response, (StatusCode, Json<CustomResponse>)> {
    let user = state.repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("Check your information"),
            }),
        ))?;

    let matching_res = HashService::check_password(&body.password, &user.password)
        .map_err(|_| (
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("Check your information"),
            }),
        ))?;

    if !matching_res {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(CustomResponse {
                message: String::from("Check your information"),
            }),
        ));
    }

    let token = JwtService::generate_jwt(&user.email).map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(CustomResponse {
            message: err.to_string(),
        }),
    ))?;

    let expiration_date = OffsetDateTime::now_utc() + Duration::days(20);
    let cookie = cookie::Cookie::build(("Authorization", token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(Expiration::DateTime(expiration_date))
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
    let claims = JwtService::verify_jwt(token)
        .map_err(|err| (
            StatusCode::UNAUTHORIZED,
            Json(CustomResponse {
                message: err.to_string(),
            }),
        ))?;

    state.repository
        .find_user_by_email(&claims.sub)
        .await
        .map_err(|_| (
            StatusCode::UNAUTHORIZED,
            Json(CustomResponse {
                message: String::from("Unauthorized"),
            }),
        ))?;

    Ok(Json(CustomResponse {
        message: String::from("Authorized")
    }))
}

pub async fn check_cookie(State(state): State<AppState>, headers: HeaderMap) -> Result<Response, (StatusCode, Json<CustomResponse>)> {
    let cookie = match extract_auth_cookie(headers) {
        Ok(c) => c,
        Err(err) => return err,
    };

    let token = cookie::Cookie::parse(cookie)
        .map_err(|err| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomResponse {
                message: err.to_string(),
            }),
        ))?;

    let claims = JwtService::verify_jwt(token.value())
        .map_err(|err| (
            StatusCode::UNAUTHORIZED,
            Json(CustomResponse {
                message: err.to_string(),
            }),
        ))?;

    state.repository.find_user_by_email(&claims.sub).await
        .map_err(|_| (
            StatusCode::UNAUTHORIZED,
            Json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        ))?;

    Ok(Json(CustomResponse {
        message: String::from("Authorized")
    }).into_response())
}

pub async fn logout(headers: HeaderMap) -> Result<Response, (StatusCode, Json<CustomResponse>)> {
    let cookie = match extract_auth_cookie(headers) {
        Ok(c) => c,
        Err(err) => return err,
    };

    let token = cookie::Cookie::parse(cookie).map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(CustomResponse {
            message: err.to_string(),
        }),
    ))?;

    JwtService::verify_jwt(token.value())
        .map_err(|err| (
            StatusCode::UNAUTHORIZED,
            Json(CustomResponse {
                message: err.to_string(),
            }),
        ))?;

    let cookie = cookie::Cookie::build(("Authorization", ""))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(OffsetDateTime::now_utc())
        .build();

    let response = (
        [(SET_COOKIE, cookie.to_string())], // headers
        Json(CustomResponse { message: String::from("Successfully logged out !") }) // body
    ).into_response();

    Ok(response)
}

#[allow(dead_code)]
fn extract_auth_cookie(headers: HeaderMap) -> Result<String, Result<Response, (StatusCode, Json<CustomResponse>)>> {
    let cookie = match headers.get("cookie") {
        Some(c) => match c.to_str() {
            Ok(c) => c,
            Err(err) => {
                return Err(Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(CustomResponse {
                        message: err.to_string(),
                    }),
                )));
            }
        },
        None => {
            return Err(Err((
                StatusCode::UNAUTHORIZED,
                Json(CustomResponse {
                    message: String::from("Unauthorized"),
                }),
            )));
        }
    };

    Ok(cookie.to_owned())
}