use actix_web::{get, Responder, web, HttpResponse};
use sea_orm::DbConn;
use oso::Oso;

use crate::auth::JwtData;
use crate::db::user_db::*;
use crate::domain::response::UserResponse;
use crate::error_responses::{e500, e404, e403};


#[get("/me")]
async fn get_user_details_handler(
    jwt_data: web::ReqData<JwtData>,
    oso: web::Data<Oso>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;
    let jwt_data = jwt_data.into_inner();

    let user = select_user_from_id(jwt_data.user_id.clone(), db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("fail", "User with that ID not found", "NotFound"))?;

    if !oso.is_allowed(jwt_data, "view", user.clone()).map_err(|e| e500("error", "Unexpected server error occured", e))? {
        return Err(e403("fail", "User does not have permission", "Forbidden"));
    }

    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}