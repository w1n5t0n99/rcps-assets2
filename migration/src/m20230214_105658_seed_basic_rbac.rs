use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};
use ::entity::{permissions, roles, roles_permissions};


static PERMS: [&'static str; 8] = ["user_view", "user_edit", "user_create", "apply__item_action", "item_view", "item_edit", "item_create", "edit_settings"];

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
            id: Set("admin".to_string())
        }
        .insert(&transaction)
        .await?;

        roles::ActiveModel {
            id: Set("inactive".to_string())
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
            perm_id: Set("user_view".to_string()),
            role_id: Set("inactive".to_string())
        }
        .insert(&transaction)
        .await?;

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
