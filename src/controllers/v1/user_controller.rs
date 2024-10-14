use crate::config::roles::Role;
use crate::controllers::{AppState, CustomResponse};
use crate::repository::user_repository::NewUser;
use crate::services::access_control::Authorization::{Authorized, Unauthorized};
use crate::services::access_control::GrantAccess;
use crate::services::crypto::{Hash, HashService};
use actix_web::{delete, get, patch, post, put, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUserBody {
    email: String,
    password: String,
}

#[get("/user/ban/{email}")]
pub async fn get_ban_user_by_email(
    state: web::Data<AppState>,
    email: web::Path<String>,
) -> impl Responder {
    if email.as_str() == "" {
        return HttpResponse::BadRequest().json(CustomResponse {
            message: String::from("route cannot be empty"),
        });
    };

    match state.repository.get_delete_user_by_email(&email).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("/user")]
pub async fn save_user(state: web::Data<AppState>, body: web::Json<NewUserBody>) -> impl Responder {
    let hash = HashService::hash_password(&body.password)
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    let user = NewUser {
        email: body.email.clone(),
        password: hash,
        role: vec![Role::USER.to_string()],
    };

    let new_user = state
        .repository
        .save_user(user)
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(new_user)
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

#[get("/user/{email}")]
pub async fn get_user_by_email(
    state: web::Data<AppState>,
    email: web::Path<String>,
) -> impl Responder {
    if email.as_str() == "" {
        return HttpResponse::BadRequest().json(CustomResponse {
            message: String::from("route cannot be empty"),
        });
    };

    let user = state
        .repository
        .find_user_by_email(&email)
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(UserResponse {
        id: user.id,
        email: user.email,
    })
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    email: String,
    password: String,
}

#[put("/user")]
pub async fn update_password(
    state: web::Data<AppState>,
    body: web::Json<ChangePasswordRequest>,
) -> impl Responder {
    let user = state
        .repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        })
        .unwrap();

    let hash = HashService::hash_password(&body.password)
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    state
        .repository
        .update_user_password(&user.id, &hash)
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(CustomResponse {
        message: String::from("Password updated successfully!"),
    })
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUserRequest {
    email: String,
}

#[patch("/user/delete")]
pub async fn soft_delete_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<DeleteUserRequest>,
) -> impl Responder {
    match state
        .access_control
        .with_cookie(req.headers().get("cookie"), vec![Role::ADMIN])
        .await
    {
        Authorized => {}
        Unauthorized(_) => {
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    }

    let user = state
        .repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        })
        .unwrap();

    state
        .repository
        .soft_delete_user(&user.id)
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(CustomResponse {
        message: String::from("User deleted successfully!"),
    })
}

#[patch("/user/undelete")]
pub async fn remove_soft_deletion_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<DeleteUserRequest>,
) -> impl Responder {
    match state
        .access_control
        .with_cookie(req.headers().get("cookie"), vec![Role::ADMIN])
        .await
    {
        Authorized => {}
        Unauthorized(_) => {
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    }

    let user = state
        .repository
        .find_banned_user_by_email(&body.email)
        .await
        .map_err(|_| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        })
        .unwrap();

    state
        .repository
        .remove_soft_deletion_user(&user.id)
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(CustomResponse {
        message: String::from("User is now accessible!"),
    })
}

#[delete("/user")]
pub async fn hard_delete_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<DeleteUserRequest>,
) -> impl Responder {
    match state
        .access_control
        .with_cookie(req.headers().get("cookie"), vec![Role::ADMIN])
        .await
    {
        Authorized => {}
        Unauthorized(_) => {
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    }

    let user = state
        .repository
        .find_user_by_email(&body.email)
        .await
        .map_err(|_| {
            HttpResponse::BadRequest().json(CustomResponse {
                message: String::from("This user doesn't exist"),
            })
        })
        .unwrap();

    state
        .repository
        .hard_delete_user(&user.id)
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(CustomResponse {
        message: String::from("User deleted successfully!"),
    })
}

#[get("/stats/user-progression")]
pub async fn get_user_progression(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    match state
        .access_control
        .with_cookie(req.headers().get("cookie"), vec![Role::ADMIN])
        .await
    {
        Authorized => {}
        Unauthorized(_) => {
            return HttpResponse::Unauthorized().json(CustomResponse {
                message: String::from("Unauthorized"),
            })
        }
    }

    let res = state
        .repository
        .get_v_user_progression()
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().json(CustomResponse {
                message: err.to_string(),
            })
        })
        .unwrap();

    HttpResponse::Ok().json(res)
}
