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
    user.password_change = Set(false);
    user.updated_at = Set(chrono::offset::Utc::now().naive_utc());
    user.update(db).await?;    

    Ok(())
}

pub async fn find_user_roles(user_id: uuid::Uuid, db: &DbConn) -> Result<(user::Model, Vec<String>), DbErr> {
    let user_roles = User::find_by_id(user_id)
        .find_also_related(Roles)
        .one(db)
        .await?;

        if let Some((user, Some(roles))) = &user_roles {
            let perms = Roles::find_by_id(roles.id.to_owned())
                .find_also_related(Permissions)
                .all(db)
                .await?;
    
            let permissions: Vec<String> = perms.iter()
                .filter_map(|(_, perm)| 
                    match perm {
                        Some(perm) => Some(perm.id.to_string()),
                        None => None,            
                    }
                )
                .collect();

            return Ok((user.clone(), permissions))
        }

    Err(DbErr::RecordNotFound("no user or roles found".to_string()))
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
