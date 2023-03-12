use actix_web::{post, Responder, web, HttpResponse};
use chrono::Duration;
use jsonwebtoken::EncodingKey;
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;
use serde_json::json;

use crate::error_responses::*;
use crate::auth::password::{Credentials, validate_credentials};
use crate::auth::jwt::{generate_jwt_from_user, generate_jwt_cookie};


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
#[post("/login")]
async fn login_user_handler(
    body: web::Json<LoginUserModel>,
    db_conn: web::Data<DbConn>,
    encoding_key: web::Data<EncodingKey>,
) -> Result<impl Responder, actix_web::Error> {

    let credentials = Credentials {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    let user = validate_credentials(credentials, &db_conn)
        .await
        .map_err(|e| e400("fail", "Invalid email or password", e))?;

    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));

    let token = generate_jwt_from_user(user.id, user.organization_id, Duration::minutes(120), &encoding_key)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let token_cookie = generate_jwt_cookie(token.clone(), Duration::minutes(120));

    // TODO: split login via cookie and session token creation into seperate api routes
    Ok(HttpResponse::Ok()
        .cookie(token_cookie)
        .json(json!({"status": "success", "token": token})))
}