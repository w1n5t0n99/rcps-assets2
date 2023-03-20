use sea_orm::*;
use secrecy::Secret;

use super::organization_db;
use super::user_db;

use domain::request::RegistrationBody;
use ::entity::user;
use ::entity::organization;


#[derive(Debug, Clone)]
pub struct InsertRegistrationModel {
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub organization: String,
}

impl InsertRegistrationModel {
    pub fn from_registration_model(model: RegistrationBody, password_hash: Secret<String>) -> Self {
        InsertRegistrationModel { name: model.name, email: model.email, password_hash, organization: model.organization }
    }
}

pub async fn insert_registration_data(model: InsertRegistrationModel, db: &DbConn) -> Result<(organization::Model, user::Model), DbErr> {
    let transaction = db.begin().await?;

    let org = organization_db::insert_organization(model.organization, &transaction).await?; 
    let user = user_db::insert_owner(model.name, model.email, model.password_hash, org.id, &transaction).await?;
    
    transaction.commit().await?;
    Ok((org, user))
}

