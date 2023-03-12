use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;

use crate::auth::JwtData;
use crate::db::user_db::*;
use crate::auth::password::compute_password_hash_nonblocking;
use crate::domain::response::UserResponse;
use crate::error_responses::*;
use crate::utils::{DbErrbExt, spawn_blocking_with_tracing};


#[derive(Debug, Deserialize)]
struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "create user", skip_all, fields(email=tracing::field::Empty))]
#[post("")]
async fn create_user_handler(
    jwt_data: web::ReqData<JwtData>,
    body: web::Json<CreateUserModel>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    tracing::Span::current().record("email", &tracing::field::display(&body.email));
    let db_conn: &DbConn = &*db_conn;

    let password_hash = compute_password_hash_nonblocking(body.password.clone())
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

        
    let user = insert_user(body.name.clone(), body.email.clone(), password_hash, jwt_data.org_id, db_conn)
        .await
        .map_err(|e| {
            if e.is_unique_key_constraint() {
                e409("fail", "User with that email already exists", "AuthError")
            }
            else {
                e500("error", "Unexpected server error occured", e)
            }
        })?;


    // TODO: should return organization data not user data
    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}