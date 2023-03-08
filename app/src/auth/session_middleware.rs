use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::web::{Data, ReqData};
use actix_web::{FromRequest, HttpMessage};
use actix_web_lab::middleware::Next;
use sea_orm::DbConn;

