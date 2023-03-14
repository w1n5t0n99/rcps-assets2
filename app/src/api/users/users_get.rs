use actix_web::{get, Responder, web, HttpResponse};
use sea_orm::DbConn;

use crate::auth::JwtData;
use crate::db::user_db;
use crate::domain::response::UsersResponse;
use crate::error_responses::*;


#[tracing::instrument(name = "users get", skip_all)]
#[get("")]
async fn gets_users_handler(
    jwt_data: web::ReqData<JwtData>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
        
    // Only want to retrieve users that are apart of the same orgainization as current user
    let users = user_db::select_users(jwt_data.org_id, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    Ok(HttpResponse::Ok().json(UsersResponse::new("success", users)))
}