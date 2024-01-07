use axum::Json;

pub async fn save_user() -> Result<Json<String>, ()> {
    Ok(Json("Hello".to_string()))
}