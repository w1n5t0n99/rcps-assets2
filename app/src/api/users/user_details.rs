use actix_web::{get, Responder, web, HttpResponse};
use sea_orm::DbConn;

use crate::auth::ApiClient;
use crate::auth::authorize::Authorize;
use crate::db::user_db::*;
use crate::error_responses::{e500, e404};

use domain::response::UserResponse;


#[tracing::instrument(name = "current user details", skip_all, fields(user_id = client.user_id.to_string()))]
#[get("/me")]
async fn get_user_details_handler(
    client: web::ReqData<ApiClient>,
    authorize: web::Data<Authorize>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
    let client = client.into_inner();

    let user = select_user_from_id(client.user_id.clone(), db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("fail", "User with that ID not found", "NotFound"))?;

    authorize.is_allowed_or_forbidden(client, "view", user.clone())?;

    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}