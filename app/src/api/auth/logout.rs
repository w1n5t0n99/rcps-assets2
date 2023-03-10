use actix_web::{get, Responder, HttpResponse, cookie::Cookie, cookie::time::Duration};
use serde_json::json;

use crate::auth::jwt_middleware::JwtMiddleware;


#[get("/auth/logout")]
async fn logout_handler( _: JwtMiddleware) -> impl Responder {
    // Set expired cookie to force logout 
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(Duration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

