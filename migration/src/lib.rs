pub use sea_orm_migration::prelude::*;

mod m20230213_145127_create_user_table;
mod m20230214_094845_create_permissions_table;
mod m20230214_102737_create_roles_table;
mod m20230214_104118_create_roles_permissions_table;
mod m20230214_105658_seed_basic_rbac;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230213_145127_create_user_table::Migration),
            Box::new(m20230214_094845_create_permissions_table::Migration),
            Box::new(m20230214_102737_create_roles_table::Migration),
            Box::new(m20230214_104118_create_roles_permissions_table::Migration),
            Box::new(m20230214_105658_seed_basic_rbac::Migration),
        ]
    }
}
