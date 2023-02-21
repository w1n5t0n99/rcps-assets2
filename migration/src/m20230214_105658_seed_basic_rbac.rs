use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};
use ::entity::{permissions, roles, roles_permissions};


static PERMS: [&'static str; 12] = [
    "user_view",
    "user_edit",
    "user_create",
    "item_assign_user",
    "item_assign_location",
    "item_view",
    "item_edit",
    "item_create",
    "view_settings",
    "edit_settings",
    "profile_view",
    "profile_edit",
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

        // insert roles
        roles::ActiveModel {
            id: Set("admin".to_string()),
            description: Set("Administrator Role - All Permissions".to_owned()),
            created_at: Set(chrono::offset::Utc::now().naive_utc()),
            updated_at: Set(chrono::offset::Utc::now().naive_utc()),
            is_admin: Set(true),
        }
        .insert(&transaction)
        .await?;

        roles::ActiveModel {
            id: Set("inactive".to_string()),
            description: Set("Inactive Role - No Permissions".to_owned()),
            created_at: Set(chrono::offset::Utc::now().naive_utc()),
            updated_at: Set(chrono::offset::Utc::now().naive_utc()),
            is_admin: Set(false),
        }
        .insert(&transaction)
        .await?;

        // insert role/permissions
        for p in PERMS {
            roles_permissions::ActiveModel {
                perm_id: Set(p.to_string()),
                role_id: Set("admin".to_string())
            }
            .insert(&transaction)
            .await?;
        }

        roles_permissions::ActiveModel {
            perm_id: Set("profile_view".to_string()),
            role_id: Set("inactive".to_string())
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

       Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // prevent from returning error on rollback
        Ok(())
    }
}
