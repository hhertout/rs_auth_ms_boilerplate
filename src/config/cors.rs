use axum::http::{HeaderValue, Method};
use axum::http::header::CONTENT_TYPE;
use tower_http::cors::{AllowCredentials, AllowHeaders, Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    let allow_origin = std::env::var("CORS_ALLOW_ORIGIN").unwrap();
    match allow_origin.as_str() {
        "*" => CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
            .allow_origin(Any)
            .allow_headers(AllowHeaders::list([CONTENT_TYPE])),
        _ => CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
            .allow_origin(allow_origin.as_str().parse::<HeaderValue>().unwrap())
            .allow_headers(AllowHeaders::list([CONTENT_TYPE]))
            .allow_credentials(AllowCredentials::yes())
    }
}