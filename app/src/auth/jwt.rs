use serde::{Serialize, Deserialize};

use crate::auth::password;


#[derive(Debug, Deserialize)]
pub struct RegisterUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

