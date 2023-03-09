use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct LoginUserModel {
    pub email: String,
    pub password: Secret<String>,
}


#[tracing::instrument(
    name = "login",
    skip_all,
    fields(email=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserModel>,
    data: web::Data<DbConn>,
) -> impl Responder {


    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "MESSAGE"}))
}