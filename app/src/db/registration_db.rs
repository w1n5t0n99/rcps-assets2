use sea_orm::*;
use secrecy::Secret;

use super::organization_db;
use super::user_db;
use ::entity::user;
use ::entity::organization;

pub async fn insert_registration_data(name: String, email: String, password_hash: Secret<String>, org_name: String, db: &DbConn) -> Result<(organization::Model, user::Model), DbErr> {
    let transaction = db.begin().await?;

    let org = organization_db::insert_organization(org_name, &transaction).await?; 
    let user = user_db::insert_owner(name, email, password_hash, org.id, &transaction).await?;
    
    transaction.commit().await?;
    Ok((org, user))
}

