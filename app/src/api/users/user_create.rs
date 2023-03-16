use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;

use crate::auth::ApiClient;
use crate::db::user_db::*;
use crate::auth::password::compute_password_hash_nonblocking;
use crate::domain::response::UserResponse;
use crate::domain::body::{CreateUserBody, CreateSecureUserModel};
use crate::error_responses::*;
use crate::utils::DbErrbExt;


#[tracing::instrument(name = "create user", skip_all, fields(email=tracing::field::Empty))]
#[post("")]
async fn create_user_handler(
    client: web::ReqData<ApiClient>,
    body: web::Json<CreateUserBody>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    tracing::Span::current().record("email", &tracing::field::display(&body.user.email));
    let db_conn: &DbConn = &*db_conn;

    let password_hash = compute_password_hash_nonblocking(body.user.password.clone())
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let model = CreateSecureUserModel::from_user_model(body.user.clone(), password_hash);

    if client.role.ne("admin") {
        return Err(e403("fail", "User does not have permission", "Forbidden"));
    }
        
    let user = insert_user(model, client.org_id, db_conn)
        .await
        .map_err(|e| {
            if e.is_unique_key_constraint() {
                e409("fail", "User already exists", "AuthError")
            }
            else {
                e500("error", "Unexpected server error occured", e)
            }
        })?;


    Ok(HttpResponse::Ok().json(UserResponse::new("success", user)))
}