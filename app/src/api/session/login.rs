use actix_web::{post, Responder, web, HttpResponse};
use chrono::Duration;
use domain::response::UserLoginResponse;
use jsonwebtoken::EncodingKey;
use sea_orm::DbConn;
use secrecy::Secret;
use serde::Deserialize;
use serde_json::json;

use crate::error_responses::*;
use crate::auth::password::{select_user_with_valid_credentials, PasswordError};
use crate::auth::jwt::{generate_jwt_from_user, generate_jwt_cookie};


#[derive(Debug, Deserialize)]
pub struct CredentialsBody {
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
    body: web::Json<CredentialsBody>,
    db_conn: web::Data<DbConn>,
    encoding_key: web::Data<EncodingKey>,
) -> Result<impl Responder, actix_web::Error> {

    tracing::Span::current().record("email", &tracing::field::display(&body.email));
    //TODO: handle user that already has valid token
    let user = select_user_with_valid_credentials(body.email.as_str(), body.password.clone(), &db_conn)
        .await
        .map_err(|e|
             match e {
                PasswordError::InvalidCredentials(_) => e400("fail", "Invalid email or password", e),
                _ => e500("error", "Unexpected server error occured", e),
             }
        )?;

    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));

    let token = generate_jwt_from_user(user.id, user.organization_id, user.role.into(), Duration::minutes(120), &encoding_key)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let token_cookie = generate_jwt_cookie(token.clone(), Duration::minutes(120));

    Ok(HttpResponse::Ok()
        .cookie(token_cookie)
        .json(json!(UserLoginResponse::new("Success", token))))
}