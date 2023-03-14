use sea_orm::*;
use ::entity::user;
use ::entity::prelude::User;
use secrecy::{Secret, ExposeSecret};
use uuid::Uuid;


pub async fn select_user_from_email<C: ConnectionTrait>(email: &str, db: &C) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

pub async fn select_user_from_id<C: ConnectionTrait>(user_id: Uuid, db: &C) -> Result<Option<user::Model>, DbErr> {
    User::find_by_id(user_id)
        .one(db)
        .await
}

pub async fn select_users<C: ConnectionTrait>(org_id: Uuid, db: &C) -> Result<Vec<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::OrganizationId.eq(org_id))
        .all(db)
        .await
}

pub async fn update_user_password<C: ConnectionTrait>(user_id: uuid::Uuid, password_hash: Secret<String>, db: &C) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id).one(db).await?;
    let mut user: user::ActiveModel = user.unwrap().into();
    
    user.password_hash = Set(password_hash.expose_secret().to_owned());
    user.updated_at = Set(chrono::offset::Utc::now().into());
    user.update(db).await?;    

    Ok(())
}

pub async fn insert_user<C: ConnectionTrait>(name: String, email: String, password_hash: Secret<String>, org_id: Uuid, db: &C) -> Result<user::Model, DbErr> {
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name),
        email: Set(email),
        password_hash: Set(password_hash.expose_secret().to_string()),
        created_at: Set(chrono::offset::Utc::now().into()),
        updated_at: Set(chrono::offset::Utc::now().into()),
        is_owner: Set(false),
        organization_id: Set(org_id),
        ..Default::default()
    };   

    user.insert(db).await
}

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
        ..Default::default()
    };   

    user.insert(db).await
}

pub async fn update_user<C: ConnectionTrait>(user: user::Model, name: Option<String>, db: &C) -> Result<user::Model, DbErr> {
    let mut user: user::ActiveModel = user.into();

    if let Some(name) = name {
        user.name = Set(name);
    }

    let user = user.update(db).await;
    user
}