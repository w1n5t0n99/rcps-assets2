use secrecy::Secret;
use serde::{Deserialize, Serialize};

use super::RoleModel;


#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
    pub role: RoleModel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserBody {
    pub user: CreateUserModel,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserModel {
    pub name: Option<String>,
    pub role: Option<RoleModel>
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserBody {
    pub user: UpdateUserModel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrationBody {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
    pub organization: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CredentialsBody {
    pub email: String,
    pub password: Secret<String>,
}

