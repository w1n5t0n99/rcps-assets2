use sea_orm_migration::{prelude::*, sea_orm::{DeriveActiveEnum, EnumIter}, sea_query::extension::postgres::TypeCreateStatement};
use sea_orm_migration::sea_orm::ColumnTypeTrait;

enum Role {
    Type,
    Admin,
    Manager,
    Member,
}

impl Iden for Role {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "role",
                Self::Admin => "admin",
                Self::Manager => "manager",
                Self::Member => "member",
            }
        )
        .unwrap();
    }
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.create_type(
            TypeCreateStatement::new()
                .as_enum(Role::Type)
                .values([Role::Admin, Role::Manager, Role::Member])
                .to_owned()
        )
        .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Photo).string().not_null().default("default.png"))
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(User::IsOwner).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(User::Role).enumeration(Role::Type, [Role::Admin, Role::Manager, Role::Member]).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_user_organization")
                            .from(User::Table, User::OrganizationId)
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Photo,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
    IsOwner,
    OrganizationId,
    Role,
}

#[derive(Iden)]
enum Organization {
    Table,
    Id,
}

