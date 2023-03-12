use actix_web::{get, Responder, web, HttpResponse};
use sea_orm::DbConn;
use uuid::Uuid;

use crate::auth::JwtData;
use crate::db::user_db::*;
use crate::domain::response::UserResponse;
use crate::error_responses::{e500, e404};


#[get("/me")]
async fn get_user_details_handler(
    jwt_data: web::ReqData<JwtData>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    let db_conn: &DbConn = &*db_conn;

    let user = select_user_from_id(jwt_data.user_id, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("fail", "User with that ID not found", "NotFound"))?;


    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}