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
    let user = match state.repository.find_user_by_email(&body.email).await {
        Ok(user) => user,
        Err(err) => {
            log::error!("{:?}", err);
            return HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("Check your information"),
            });
        }
    };

    match HashService::check_password(&body.password, &user.password) {
        Ok(valid) => {
            if !valid {
                return HttpResponse::BadRequest().json(CustomResponse {
                    message: String::from("Check your information"),
                });
            }
        }
        Err(err) => {
            log::error!("{:?}", err);
            return HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("Check your information"),
            });
        }
    };

    let token = JwtService::generate_jwt(&user.email)
        .map_err(|err| {
            log::error!("{:?}", err);
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
    let token = match req.headers().get("Authorization") {
        Some(t) => t,
        None => {
            log::error!("No Authorization header found");
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            });
        }
    };

    let claims = match JwtService::verify_jwt(token.to_str().unwrap()) {
        Ok(c) => c,
        Err(err) => {
            log::error!("{:?}", err);
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: format!("JWT verification error: {}", err),
            });
        }
    };

    match state.repository.find_user_by_email(&claims.sub).await {
        Ok(_) => HttpResponse::Ok().json(CustomResponse {
            message: String::from("Authorized"),
        }),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    }
}

#[get("/cookie/check")]
pub async fn check_cookie(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let cookie = match extract_auth_cookie(req.headers().get("cookie")) {
        Ok(c) => c,
        Err(err) => {
            log::error!("{:?}", err);
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            });
        }
    };

    let token = Cookie::parse(cookie)
        .map_err(|err| {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    let claims = JwtService::verify_jwt(token.value())
        .map_err(|err| {
            log::error!("{:?}", err);
            HttpResponse::Unauthorized().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    match state.repository.find_user_by_email(&claims.sub).await {
        Ok(_) => HttpResponse::Ok().json(CustomResponse {
            message: String::from("Authorized"),
        }),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    }
}

#[get("/logout")]
pub async fn logout(_: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let cookie = match extract_auth_cookie(req.headers().get("cookie")) {
        Ok(c) => c,
        Err(err) => {
            log::error!("{:?}", err);
            return HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("No cookie provided"),
            });
        }
    };

    match Cookie::parse(cookie) {
        Ok(token) => match JwtService::verify_jwt(token.value()) {
            Ok(_) => {
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
            Err(err) => {
                log::error!("{:?}", err);
                return HttpResponse::Unauthorized().json(CustomResponse {
                    message: err.to_string(),
                });
            }
        },
        Err(err) => {
            log::error!("{:?}", err);
            return HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            });
        }
    }
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
