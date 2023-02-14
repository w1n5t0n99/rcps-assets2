use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};
use ::entity::permissions;


static PERMS: [&'static str; 8] = ["user_view", "user_edit", "user_create", "apply__item_action", "item_view", "item_edit", "item_create", "edit_settings"];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let transaction = db.begin().await?;

        for p in PERMS {
            permissions::ActiveModel {
                id: Set(p.to_string()),
            }
            .insert(&transaction)
            .await?;
        }

        transaction.commit().await?;

       Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum RolesPermissionsIden {
    Table,
    RoleID,
    PermID,
}

#[derive(Iden)]
enum RolesIden {
    Table,
    Id,
}

#[derive(Iden)]
enum PermissionsIden {
    Table,
    Id,
}
