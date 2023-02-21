use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Description).string().not_null().default(" "))
                    .col(ColumnDef::new(Roles::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Roles::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Roles::IsAdmin).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Roles {
    Table,
    Id,
    Description,
    CreatedAt,
    UpdatedAt,
    IsAdmin,
}
