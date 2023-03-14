use secrecy::Secret;
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserBody {
    pub user: CreateUserModel,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserModel {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserBody {
    pub user: UpdateUserModel,
}