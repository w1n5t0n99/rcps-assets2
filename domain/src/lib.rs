pub mod response;
pub mod request;

use entity::sea_orm_active_enums::Role;


#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RoleModel {
    Admin,
    Manager,
    Member,
}

impl From<RoleModel> for Role {
    fn from(r: RoleModel) -> Self {
        match r {
            RoleModel::Admin => Self::Admin,
            RoleModel::Manager => Self::Manager,
            RoleModel::Member => Self::Member,
        }
    }
}

impl From<Role> for RoleModel {
    fn from(r: Role) -> Self {
        match r {
            Role::Admin => Self::Admin,
            Role::Manager => Self::Manager,
            Role::Member => Self::Member,
        }
    }
}