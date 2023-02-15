pub use sea_orm_migration::prelude::*;

mod m20230213_145127_create_user_table;
mod m20230214_094845_create_permissions_table;
mod m20230214_102737_create_roles_table;
mod m20230214_104118_create_roles_permissions_table;
mod m20230214_105658_seed_basic_rbac;
mod m20230214_145955_alter_users_add_role_fk;
mod m20230215_092319_seed_admin_user;

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
            Box::new(m20230214_145955_alter_users_add_role_fk::Migration),
            Box::new(m20230215_092319_seed_admin_user::Migration),
        ]
    }
}
