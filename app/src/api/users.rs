use actix_web::{get, Responder, web, HttpResponse};
use sea_orm::DbConn;
use uuid::Uuid;

use crate::auth::jwt_middleware::JwtMiddleware;
use crate::db::user_db::*;
use crate::domain::response::filter_user_record;
use crate::error_responses::{e500, e404};


#[get("/users/me")]
async fn get_me_handler(
    user_id: web::ReqData<Uuid>,
    db_conn: web::Data<DbConn>,
    _: JwtMiddleware,
) -> Result<impl Responder, actix_web::Error> {

    let user = select_user_from_id(*user_id, &db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?
        .ok_or_else(|| e404("fail", "User with that ID not found", "NotFound"))?;

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    Ok(HttpResponse::Ok().json(json_response))
}