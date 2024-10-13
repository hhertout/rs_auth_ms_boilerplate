use crate::controllers::{AppState, CustomResponse};
use crate::services::crypto::Hash;
use crate::services::crypto::Jwt;
use crate::services::crypto::{CSRFTokenService, HashService, JwtService};
use actix_web::http::header::{HeaderValue, SET_COOKIE};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use cookie::time::{Duration, OffsetDateTime};
use cookie::{Cookie, Expiration, SameSite};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
    email: String,
    role: Vec<String>,
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginBody>) -> impl Responder {
    let user = state
        .repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("Check your information"),
            })
        })
        .unwrap();

    let matching_res = HashService::check_password(&body.password, &user.password)
        .map_err(|_| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("Check your information"),
            })
        })
        .unwrap();

    if !matching_res {
        return HttpResponse::BadRequest().json(CustomResponse {
            message: String::from("Check your information"),
        });
    }

    let token = JwtService::generate_jwt(&user.email)
        .map_err(|err| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    let expiration_date = OffsetDateTime::now_utc() + Duration::days(20);
    let cookie = cookie::Cookie::build(("Authorization", &token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(Expiration::DateTime(expiration_date))
        .build();

    HttpResponse::Ok()
        .append_header((SET_COOKIE, cookie.to_string()))
        .json(LoginResponse {
            token,
            email: user.email,
            role: user.role,
        })
}

#[derive(Serialize, Deserialize)]
pub struct CheckTokenBody {
    token: String,
}

#[get("/token/check")]
pub async fn check_token(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let token = match extract_auth_cookie(req.headers().get("cookie")) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    };

    let claims = JwtService::verify_jwt(&token)
        .map_err(|err| {
            HttpResponse::Unauthorized().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    state
        .repository
        .find_user_by_email(&claims.sub)
        .await
        .map_err(|_| {
            HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(CustomResponse {
        message: String::from("Authorized"),
    })
}

#[get("/cookie/check")]
pub async fn check_cookie(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let cookie = match req.headers().get("Authorization") {
        Some(c) => c.to_str().unwrap(),
        None => {
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    };

    let token = Cookie::parse(cookie)
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    let claims = JwtService::verify_jwt(token.value())
        .map_err(|err| {
            HttpResponse::Unauthorized().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    state
        .repository
        .find_user_by_email(&claims.sub)
        .await
        .map_err(|_| {
            HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(CustomResponse {
        message: String::from("Authorized"),
    })
}

#[post("/logout")]
pub async fn logout(_: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let cookie = match extract_auth_cookie(req.headers().get("cookie")) {
        Ok(c) => c,
        Err(err) => return err,
    };

    let token = Cookie::parse(cookie)
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    JwtService::verify_jwt(token.value())
        .map_err(|err| {
            HttpResponse::Unauthorized().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    let cookie = Cookie::build(("Authorization", ""))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(OffsetDateTime::now_utc())
        .build();

    HttpResponse::Ok()
        .append_header((SET_COOKIE, cookie.to_string()))
        .json(CustomResponse {
            message: String::from("Successfully logged out!"),
        })
}

#[get("/csrf-token")]
pub async fn get_csrf_token() -> impl Responder {
    let token = CSRFTokenService::generate_csrf_token()
        .map_err(|_| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: String::from("Internal server error"),
            })
        })
        .unwrap();

    let expiration_date = OffsetDateTime::now_utc() + Duration::days(1);
    let cookie = Cookie::build(("XSRF-TOKEN", token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(expiration_date)
        .build();

    HttpResponse::Ok()
        .append_header((SET_COOKIE, cookie.to_string()))
        .finish()
}

pub(crate) fn extract_auth_cookie(headers: Option<&HeaderValue>) -> Result<String, HttpResponse> {
    let cookie_header = match headers {
        Some(c) => c,
        None => {
            return Err(HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Cookie is not set"),
            }))
        }
    };

    let cookie = match cookie_header.to_str() {
        Ok(c) => c,
        Err(err) => {
            return Err(HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            }))
        }
    };

    for cookie in cookie.split(';') {
        if cookie.contains("Authorization") {
            let token = cookie.trim();
            return Ok(token.to_owned());
        }
    }

    Err(HttpResponse::Unauthorized().json(CustomResponse {
        message: String::from("Auth Cookie is not set"),
    }))
}
