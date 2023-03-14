use sea_orm::*;
use secrecy::Secret;

use super::organization_db;
use super::user_db;
use crate::domain::body::RegistrationSecureModel;
use ::entity::user;
use ::entity::organization;


pub async fn insert_registration_data(model: RegistrationSecureModel, db: &DbConn) -> Result<(organization::Model, user::Model), DbErr> {
    let transaction = db.begin().await?;

    let org = organization_db::insert_organization(model.organization, &transaction).await?; 
    let user = user_db::insert_owner(model.name, model.email, model.password_hash, org.id, &transaction).await?;
    
    transaction.commit().await?;
    Ok((org, user))
}

