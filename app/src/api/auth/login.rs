use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;

use crate::error_responses::*;
use crate::auth::password::{Credentials, AuthError, validate_credentials};


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
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {

    let credentials = Credentials {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    let user_id = validate_credentials(credentials, &db_conn)
        .await
        .map_err(|e| e400("fail", "Invalid email or password", e))?;

    

    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "MESSAGE"})))
}