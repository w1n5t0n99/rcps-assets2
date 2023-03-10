use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use sea_orm::DbConn;

use crate::utils::spawn_blocking_with_tracing;
use crate::db::user_db::*;
use super::AuthError;

const DEFAULT_PASSWORD_HASH: &'static str = "$argon2id$v=19$m=15000,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno";

pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();

    Ok(Secret::new(password_hash))
}

#[tracing::instrument(name = "Get stored credentials", skip_all)]
async fn get_stored_credentials(
    email: &str,
    db_conn: &DbConn,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {

    let user = select_user_from_email(email, db_conn)
        .await
        .context("Failed to lookup credentials")?;

    let user = user.map(|u| (u.id, Secret::new(u.password_hash)));
    Ok(user)
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Validate credentials", skip_all)]
pub async fn validate_credentials(
    credentials: Credentials,
    db_conn: &DbConn,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(DEFAULT_PASSWORD_HASH.to_string());

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.email, db_conn).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || { verify_password_hash(expected_password_hash, credentials.password) })
        .await
        .context("Failed to spawn blocking task.")??;

    user_id
        .context("Unknown username")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Change password", skip(password, db_conn))]
pub async fn change_password(
    user_id: uuid::Uuid,
    password: Secret<String>,
    db_conn: &DbConn,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;
    
    update_user_password(user_id, password_hash, db_conn)
        .await
        .context("Failed to change user's password in the database.")?;
    
    Ok(())
}

