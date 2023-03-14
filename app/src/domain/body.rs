use secrecy::Secret;
use serde::Deserialize;

use super::RoleModel;


#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
    pub role: RoleModel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSecureUserModel {
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub role: RoleModel,
}

impl CreateSecureUserModel {
    pub fn from_user_model(model: CreateUserModel, password_hash: Secret<String>) -> Self {
        CreateSecureUserModel { name: model.name, email: model.email, password_hash, role: model.role }
    }
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
pub struct RegistrationModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
    pub organization: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrationSecureModel {
    pub name: String,
    pub email: String,
    pub password_hash: Secret<String>,
    pub organization: String,
}

impl RegistrationSecureModel {
    pub fn from_registration_model(model: RegistrationModel, password_hash: Secret<String>) -> Self {
        RegistrationSecureModel { name: model.name, email: model.email, password_hash, organization: model.organization }
    }
}