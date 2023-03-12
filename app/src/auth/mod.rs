use serde::{Serialize, Deserialize};

pub mod password;
pub mod jwt;
pub mod jwt_middleware;


#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtData {
    pub user_id: uuid::Uuid,
    pub org_id: uuid::Uuid,
}