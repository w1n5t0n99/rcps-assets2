use actix_web::{delete, Responder, web, HttpResponse};
use sea_orm::DbConn;
use serde_json::json;
use uuid::Uuid;

use crate::auth::ApiClient;
use crate::auth::authorize::Authorize;
use crate::db::user_db;
use crate::error_responses::*;


#[tracing::instrument(name = "delete user", skip_all, fields(user_id=tracing::field::Empty))]
#[delete("/{id}")]
async fn delete_user_handler(
    client: web::ReqData<ApiClient>,
    path: web::Path<Uuid>,
    authorize: web::Data<Authorize>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
    let client = client.into_inner();
    let path = path.into_inner();

    tracing::Span::current().record("user_id", &tracing::field::display(&path));

    let user = user_db::select_user_from_id(path, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("error", "User not found", "UserError"))?;

    authorize.is_allowed_or_forbidden(client.clone(), "delete", user.clone())?;

    let delete_res = user_db::delete_user(user, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;   

    Ok(HttpResponse::Ok().json(json!({"status": "success", "rows_affected": delete_res.rows_affected})))
}