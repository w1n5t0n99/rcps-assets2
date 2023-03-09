use sea_orm::*;
use ::entity::user;
use ::entity::prelude::User;
use secrecy::{Secret, ExposeSecret};


pub async fn select_user_from_email(email: &str, db: &DbConn) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

pub async fn update_user_password(user_id: uuid::Uuid, password_hash: Secret<String>, db: &DbConn) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id).one(db).await?;
    let mut user: user::ActiveModel = user.unwrap().into();
    
    user.password_hash = Set(password_hash.expose_secret().to_owned());
    user.updated_at = Set(chrono::offset::Utc::now().into());
    user.update(db).await?;    

    Ok(())
}