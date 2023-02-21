use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};
use ::entity::{permissions, roles_permissions};


static PERMS: [&'static str; 3] = [
    "roles_view",
    "roles_edit",
    "roles_create",
];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let transaction = db.begin().await?;

        // insert permissions
        for p in PERMS {
            permissions::ActiveModel {
                id: Set(p.to_string()),
            }
            .insert(&transaction)
            .await?;
        }

        // insert role/permissions
        for p in PERMS {
            roles_permissions::ActiveModel {
                perm_id: Set(p.to_string()),
                role_id: Set("admin".to_string())
            }
            .insert(&transaction)
            .await?;
        }

        transaction.commit().await?;

       Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // prevent from returning error on rollback
        Ok(())
    }
}
