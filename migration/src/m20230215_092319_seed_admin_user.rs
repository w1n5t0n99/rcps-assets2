use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;
use ::entity::user;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // password = everythinghastostartsomewhere
        user::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            name: Set("reed elam".to_string()),
            email: Set("relam@richmond-county.k12.va.us".to_string()),
            password_hash: Set("$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8".to_owned()),
            created_at: Set(chrono::offset::Utc::now().naive_utc()),
            updated_at: Set(chrono::offset::Utc::now().naive_utc()),
            password_change: Set(true),
            role: Set("admin".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // prevent from returning error on rollback
        Ok(())
    }
}

