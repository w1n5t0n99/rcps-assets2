use actix_web::cookie::Cookie;
use anyhow::Context;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};

use crate::utils::spawn_blocking_with_tracing;


#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Encoding credentials error.")]
    Encoding(#[source]  anyhow::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub org: String,
    pub rol: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn generate_jwt_from_user(user_id: Uuid, org_id: Uuid, role: String, duration: Duration, encoding_key: &EncodingKey) -> Result<String, TokenError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + duration).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        org: org_id.to_string(),
        rol: role,
        exp,
        iat,
    };

    let encoding_key = encoding_key.clone();
    let token = spawn_blocking_with_tracing(move || encode(&Header::default(), &claims, &encoding_key))
        .await
        .context("Spawn blocking failed")?;

    let token = token.context("Failed to encode JWT").map_err(TokenError::Encoding)?;

    Ok(token)
}

pub fn generate_jwt_cookie<'c>(token: String, duration: Duration) -> Cookie<'c> {
    let cookie_dur = actix_web::cookie::time::Duration::new(duration.num_seconds(), 0);

    Cookie::build("token", token)
        .path("/")
        .max_age(cookie_dur)
        .http_only(true)
        .finish()
}