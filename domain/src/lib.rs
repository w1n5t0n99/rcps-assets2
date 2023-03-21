pub mod response;
pub mod request;

use std::fmt;

use entity::sea_orm_active_enums::Role;


#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RoleModel {
    Admin,
    Manager,
    Member,
}

impl fmt::Display for RoleModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "Admin"),
            Self::Manager => write!(f, "Manager"),
            Self::Member => write!(f, "Member"),
        }
    }
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