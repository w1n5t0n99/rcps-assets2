use actix_web::{put, Responder, web, HttpResponse};
use sea_orm::DbConn;
use uuid::Uuid;

use crate::auth::JwtData;
use crate::db::user_db;
use crate::domain::response::UserResponse;
use crate::domain::body::UpdateUserBody;
use crate::error_responses::*;


#[tracing::instrument(name = "update user", skip_all, fields(user_id=tracing::field::Empty))]
#[put("/{id}")]
async fn update_user_handler(
    jwt_data: web::ReqData<JwtData>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserBody>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
    let path = path.into_inner();

    tracing::Span::current().record("user_id", &tracing::field::display(&path));

    let user = user_db::select_user_from_id(path, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("error", "User not found", "UserError"))?;

    if jwt_data.org_id != user.organization_id {
        return Err(e403("error", "You do not have access to update this user", "UserError"));
    }

    let user = user_db::update_user(user, body.user.name.clone(), db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;    

    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}