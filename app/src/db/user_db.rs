use sea_orm::*;
use ::entity::user;
use ::entity::sea_orm_active_enums::Role;
use ::entity::prelude::User;
use secrecy::{Secret, ExposeSecret};
use uuid::Uuid;

use domain::RoleModel;
use domain::request::{CreateUserModel, UpdateUserModel};


#[derive(Debug, Clone)]
pub struct InsertUserModel {
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub role: RoleModel,
}

impl InsertUserModel {
    pub fn from_user_model(model: CreateUserModel, password_hash: Secret<String>) -> Self {
        InsertUserModel { name: model.name, email: model.email, password_hash, role: model.role }
    }
}

#[tracing::instrument(name = "db - select user from email", skip_all)]
pub async fn select_user_from_email<C: ConnectionTrait>(email: &str, db: &C) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

#[tracing::instrument(name = "db - select user from id", skip_all)]
pub async fn select_user_from_id<C: ConnectionTrait>(user_id: Uuid, db: &C) -> Result<Option<user::Model>, DbErr> {
    User::find_by_id(user_id)
        .one(db)
        .await
}

#[tracing::instrument(name = "db - select users", skip_all)]
pub async fn select_users<C: ConnectionTrait>(org_id: Uuid, db: &C) -> Result<Vec<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::OrganizationId.eq(org_id))
        .all(db)
        .await
}

#[tracing::instrument(name = "db - update user password", skip_all)]
pub async fn update_user_password<C: ConnectionTrait>(user_id: uuid::Uuid, password_hash: Secret<String>, db: &C) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id).one(db).await?;
    let mut user: user::ActiveModel = user.unwrap().into();
    
    user.password_hash = Set(password_hash.expose_secret().to_owned());
    user.updated_at = Set(chrono::offset::Utc::now().into());
    user.update(db).await?;    

    Ok(())
}

#[tracing::instrument(name = "db - insert user", skip_all)]
pub async fn insert_user<C: ConnectionTrait>(model: InsertUserModel, org_id: Uuid, db: &C) -> Result<user::Model, DbErr> {
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(model.name),
        email: Set(model.email),
        password_hash: Set(model.password_hash.expose_secret().to_string()),
        created_at: Set(chrono::offset::Utc::now().into()),
        updated_at: Set(chrono::offset::Utc::now().into()),
        is_owner: Set(false),
        organization_id: Set(org_id),
        role: Set(model.role.into()),
        ..Default::default()
    };   

    user.insert(db).await
}

#[tracing::instrument(name = "db - insert owner", skip_all)]
pub async fn insert_owner<C: ConnectionTrait>(name: String, email: String, password_hash: Secret<String>, org_id: Uuid, db: &C) -> Result<user::Model, DbErr> {
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name),
        email: Set(email),
        password_hash: Set(password_hash.expose_secret().to_string()),
        created_at: Set(chrono::offset::Utc::now().into()),
        updated_at: Set(chrono::offset::Utc::now().into()),
        is_owner: Set(true),
        organization_id: Set(org_id),
        role: Set(Role::Admin),
        ..Default::default()
    };   

    user.insert(db).await
}

#[tracing::instrument(name = "db - update user", skip_all)]
pub async fn update_user<C: ConnectionTrait>(user: user::Model, model: UpdateUserModel, db: &C) -> Result<user::Model, DbErr> {
    let mut user: user::ActiveModel = user.into();

    if let Some(name) = model.name {
        user.name = Set(name);
    }

    if let Some(role) = model.role {
        user.role = Set(role.into());
    }

    let user = user.update(db).await;
    user
}

#[tracing::instrument(name = "db - delete user", skip_all)]
pub async fn delete_user<C: ConnectionTrait>(user: user::Model, db: &C) -> Result<DeleteResult, DbErr> {
    user.delete(db).await
}