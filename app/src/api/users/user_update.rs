use actix_web::{put, Responder, web, HttpResponse};
use oso::Oso;
use sea_orm::DbConn;
use uuid::Uuid;

use crate::auth::ApiClient;
use crate::auth::authorize::Authorize;
use crate::db::user_db;
use crate::error_responses::*;

use domain::response::UserResponse;
use domain::request::UpdateUserBody;


#[tracing::instrument(name = "update user", skip_all, fields(user_id=tracing::field::Empty))]
#[put("/{id}")]
async fn update_user_handler(
    client: web::ReqData<ApiClient>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserBody>,
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

    authorize.is_allowed_or_forbidden(client.clone(), "update", user.clone())?;
    // Prevent there from being no admins for acount
    if body.user.role.is_some() {
        authorize.is_allowed_or_forbidden(client.clone(), "update_role", user.clone())?;
    }

    let user = user_db::update_user(user, body.user.clone(), db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;    

    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}