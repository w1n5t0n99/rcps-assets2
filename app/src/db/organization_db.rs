use sea_orm::*;
use ::entity::organization;
use ::entity::prelude::Organization;
use uuid::Uuid;


#[tracing::instrument(name = "db - select org from id", skip_all)]
pub async fn select_organization_from_id(org_id: Uuid, db: &DbConn) -> Result<Option<organization::Model>, DbErr> {
    Organization::find_by_id(org_id)
        .one(db)
        .await
}

#[tracing::instrument(name = "db - insert org", skip_all)]
pub async fn insert_organization<C: ConnectionTrait>(name: String, db: &C) -> Result<organization::Model, DbErr> {
    let organization = organization::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name),
        created_at: Set(chrono::offset::Utc::now().into()),
        updated_at: Set(chrono::offset::Utc::now().into()),
    };   

    organization.insert(db).await
}