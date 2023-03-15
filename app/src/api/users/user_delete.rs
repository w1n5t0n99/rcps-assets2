use actix_web::{delete, Responder, web, HttpResponse};
use oso::Oso;
use sea_orm::DbConn;
use uuid::Uuid;

use crate::auth::JwtData;
use crate::db::user_db;
use crate::domain::response::UserResponse;
use crate::error_responses::*;


#[tracing::instrument(name = "delete user", skip_all, fields(user_id=tracing::field::Empty))]
#[delete("/{id}")]
async fn delete_user_handler(
    jwt_data: web::ReqData<JwtData>,
    path: web::Path<Uuid>,
    oso: web::Data<Oso>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
    let jwt_data = jwt_data.into_inner();
    let path = path.into_inner();

    tracing::Span::current().record("user_id", &tracing::field::display(&path));

    let user = user_db::select_user_from_id(path, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("error", "User not found", "UserError"))?;

    if !oso.is_allowed(jwt_data.clone(), "delete", user.clone()).map_err(|e| e500("error", "Unexpected server error occured", e))? {
        return Err(e403("fail", "User does not have permission", "Forbidden"));
    }

    user_db::delete_user(user, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;    

    Ok(HttpResponse::Ok().finish())
}