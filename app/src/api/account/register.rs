use actix_web::{post, Responder, web, HttpResponse};
use sea_orm::DbConn;

use crate::db::registration_db::*;
use crate::db::user_db;
use crate::auth::password::compute_password_hash_nonblocking;
use crate::error_responses::*;

use domain::response::RegistrationResponse;
use domain::request::RegistrationBody;


#[tracing::instrument(name = "register", skip_all, fields(email=tracing::field::Empty))]
#[post("/register")]
async fn register_account_handler(
    body: web::Json<RegistrationBody>,
    db_conn: web::Data<DbConn>,
) -> Result<impl Responder, actix_web::Error> {
    tracing::Span::current().record("email", &tracing::field::display(&body.email));
    let db_conn: &DbConn = &*db_conn;

    let exists = user_db::select_user_from_email(&body.email, db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    if let Some(_) = exists {
        return Err(e409("fail", "User with that email already exists", "AuthError"));
    }

    let password_hash = compute_password_hash_nonblocking(body.password.clone())
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let model = InsertRegistrationModel::from_registration_model(body.clone(), password_hash);
    let (org, user) = insert_registration_data(model, &db_conn)
        .await
        .map_err(|e| e500("error", "Unexpected server error occured", e))?;


    Ok(HttpResponse::Ok().json(RegistrationResponse::new("success", org, user)))
}