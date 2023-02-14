use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let fk = TableForeignKey::new()
            .name("FK_role_id")
            .from_tbl(User::Table)
            .from_col( User::Role)
            .to_tbl(Roles::Table)
            .to_col(Roles::Id)
            .on_delete(ForeignKeyAction::NoAction)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();


        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Role).string().not_null().default("inactive"))
                    .add_foreign_key(&fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .alter_table(
            Table::alter()
                .table(User::Table)
                .drop_foreign_key(Alias::new("FK_role_id"))
                .drop_column(Alias::new("role"))
                .to_owned(),
        )
        .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Role,
}

#[derive(Iden)]
enum Roles {
    Table,
    Id,
}

