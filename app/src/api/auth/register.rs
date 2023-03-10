use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;

use crate::db::user_db::*;
use crate::auth::password::compute_password_hash;
use crate::domain::response::filter_user_record;
use crate::error_responses::*;


#[derive(Debug, Deserialize)]
struct RegisterUserModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "register", skip_all, fields(email=tracing::field::Empty))]
#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserModel>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    tracing::Span::current().record("email", &tracing::field::display(&body.email));

    let exists = select_user_from_email(&body.email, &db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    if let Some(_) = exists {
        return Err(e409("fail", "User with that email already exists", "AuthError"));
    }

    let password_hash = compute_password_hash(body.password.clone())
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let user = insert_user(body.name.clone(), body.email.clone(), password_hash, &db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let user_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({"user": filter_user_record(&user)})
    });

    Ok(HttpResponse::Ok().json(user_response))
}