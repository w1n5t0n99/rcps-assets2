use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;

use crate::db::user_db::*;


#[derive(Debug, Deserialize)]
pub struct RegisterUserModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "register", skip_all, fields(email=tracing::field::Empty))]
#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserModel>,
    db_conn: web::Data<DbConn>,
) -> impl Responder {

    let exists = select_user_from_email(&body.email, &db_conn)
        .await;

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "MESSAGE"}))
}