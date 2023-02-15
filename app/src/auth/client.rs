use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
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
    pub roles: Vec<String>,
}

impl Client {
    pub async fn from_user_session(session: &TypedSession, db: &DbConn) -> Result<Client, ClientError> {
        let user_id = session.get_user_id()
            .context("Session Error")
            .map_err(ClientError::UnexpectedError)?;
    
        let user_id = user_id.ok_or_else(|| ClientError::MissingUserSession)?;
    
        //let roles = find_user_roles(user_id, db)
        //    .await
        //    .map_err(|e| ClientError::UnexpectedError(e.into()))?;
        
        Ok(Client {
            user_id,
            roles: Vec::new(),
        })
    }
}


