use sea_orm::*;
use ::entity::{user, roles, permissions, roles_permissions};
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

pub async fn find_users_with_role(db: &DbConn) -> Result<Vec<(user::Model, roles::Model)>, DbErr> {
    let users = User::find()
        .find_also_related(Roles)
        .all(db)
        .await?;
    
    let users: Vec<(user::Model, roles::Model)> = users.into_iter()
        .filter_map(|(user, role)| 
            match role {
                Some(role) => {
                    Some((user, role))
                }
                None => {
                    None
                }
            }
        )
        .collect();

    Ok(users)
}

pub async fn find_user_permissions(user_id: uuid::Uuid, db: &DbConn) -> Result<(user::Model, Vec<String>), DbErr> {
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

pub async fn find_roles(db: &DbConn) -> Result<Vec<roles::Model>, DbErr> {
    let roles = Roles::find()
        .all(db)
        .await?;

    Ok(roles)
}

pub async fn insert_role_with_permissions(db: &DbConn, name: String, description: String, perms: Vec<String>) -> Result<(), DbErr> {
    let transaction = db.begin().await?;

     // insert roles
     roles::ActiveModel {
        id: Set(name.clone()),
        description: Set(description),
        created_at: Set(chrono::offset::Utc::now().naive_utc()),
        updated_at: Set(chrono::offset::Utc::now().naive_utc()),
        is_admin: Set(false),
    }
    .insert(&transaction)
    .await?;

    // insert role/permissions
    for p in perms {
        roles_permissions::ActiveModel {
            perm_id: Set(p),
            role_id: Set(name.clone())
        }
        .insert(&transaction)
        .await?;
    }

    transaction.commit().await?;

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
