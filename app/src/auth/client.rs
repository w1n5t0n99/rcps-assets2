use crate::session_state::TypedSession;
use crate::db::*;

use anyhow::Context;
use sea_orm::DbConn;
use uuid::Uuid;


#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Missing User Session")]
    MissingUserSession,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
pub struct Client {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub permissions: Vec<String>,
    pub password_change: bool,
}

impl Client {
    pub async fn from_user_session(session: &TypedSession, db: &DbConn) -> Result<Client, ClientError> {
        let user_id = session.get_user_id()
            .context("Session Error")
            .map_err(ClientError::UnexpectedError)?;
    
        let user_id = user_id.ok_or_else(|| ClientError::MissingUserSession)?;
    
        let (user, permissions) = find_user_permissions(user_id, db)
            .await
            .map_err(|e| ClientError::UnexpectedError(e.into()))?;
        
        Ok(Client {
            user_id,
            name: user.name, 
            email: user.email,
            permissions,
            password_change: user.password_change,
        })
    }

    pub fn url_to(&self, end_point: &str) -> String {
        format!("/users/{}/{}", self.user_id, end_point)
    }

    pub fn has_permission<S: Into<String>>(&self, permission: S) -> bool {
        // TODO: benchmark may replace with HashSet
        self.permissions.contains(&permission.into())
    }
}


