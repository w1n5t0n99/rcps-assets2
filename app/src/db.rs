use sea_orm::*;
use ::entity::{user, permissions, roles_permissions};
use ::entity::prelude::{User, Roles, Permissions, RolesPermissions};
use secrecy::{Secret, ExposeSecret};

use crate::utils::{error_chain_fmt};

pub async fn find_user(email: &str, db: &DbConn) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

pub async fn update_user_password(user_id: uuid::Uuid, password_hash: Secret<String>, db: &DbConn) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id).one(db).await?;
    let mut user: user::ActiveModel = user.unwrap().into();
    
    user.password_hash = Set(password_hash.expose_secret().to_owned());
    user.update(db).await?;    

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct BulkInsert {
    pub total: usize,
    pub inserted: usize,
    pub skipped: usize,
}

#[derive(thiserror::Error)]
pub enum BulkInsertError {
    #[error("Error parsing payload file")]
    Parse(#[from] csv::Error),
    #[error("Error inserting in database")]
    Database(#[from] DbErr),
    #[error("Something went wrong")]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for BulkInsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
