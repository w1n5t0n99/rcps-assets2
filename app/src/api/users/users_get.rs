use actix_web::{get, Responder, web, HttpResponse};
use sea_orm::DbConn;

use crate::auth::authorize::Authorize;
use crate::auth::ApiClient;
use crate::db::user_db;
use crate::domain::response::UsersResponse;
use crate::error_responses::*;


#[tracing::instrument(name = "users get", skip_all)]
#[get("")]
async fn gets_users_handler(
    client: web::ReqData<ApiClient>,
    authorize: web::Data<Authorize>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
    let client = client.into_inner();

    // Only want to retrieve users that are apart of the same orgainization as current user
    let users = user_db::select_users(client.org_id, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    if let Some(user) = users.first() {
        authorize.is_allowed_or_not_found(client, "view", user.clone())?;
    }

    Ok(HttpResponse::Ok().json(UsersResponse::new("success", users)))
}