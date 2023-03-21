use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::convert::From;

use entity::user;
use entity::organization;
use super::RoleModel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub role: RoleModel,
}

impl From<user::Model> for FilteredUser {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            photo: user.photo,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: user.role.into(),
        }
    }
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub user: FilteredUser,
}

impl UserResponse {
    pub fn new<S: Into<String>, U: Into<FilteredUser>>(status: S, user: U) -> Self {
        Self {
            status: status.into(),
            user: user.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersResponse {
    pub status: String,
    pub users: Vec<FilteredUser>,
}

impl UsersResponse {
    pub fn new<S: Into<String>, U: Into<FilteredUser>>(status: S, users: Vec<U>) -> Self {
        Self {
            status: status.into(),
            users: users.into_iter().map(|u| u.into()).collect(),
        }
    }
}

// May add data later that needs to be filtered out
#[derive(Debug, Serialize, Deserialize)]
pub struct FilteredOrganization {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<organization::Model> for FilteredOrganization {
    fn from(org: organization::Model) -> Self {
        Self {
            id: org.id.to_string(),
            name: org.name,
            created_at: org.created_at,
            updated_at: org.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationResponse {
    pub status: String,
    pub org: FilteredOrganization,
    pub user: FilteredUser,
}

impl RegistrationResponse {
    pub fn new<S, O, U>(status: S,org: O, user: U) -> Self
    where 
        S: Into<String>,
        O: Into<FilteredOrganization>,
        U: Into<FilteredUser>,
    {
        Self {
            status: status.into(),
            org: org.into(),
            user: user.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub access_token: String,
}

impl UserLoginResponse {
    pub fn new<S, T>(status: S, token: T) -> Self
    where 
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            status: status.into(),
            access_token: token.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
