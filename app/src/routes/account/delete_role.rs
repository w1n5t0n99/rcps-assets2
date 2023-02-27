use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use actix_web_grants::proc_macro::has_permissions;
use sea_orm::DbConn;
use validator::{Validate, ValidateArgs, ValidationError};

use crate::db;
use crate::utils::{e500, see_other, DbErrbExt, ValidationErrorsExt};

use crate::domain::delete_role_form::DeleteRoleForm;


//=================================================================
#[post("/roles/delete")]
#[has_permissions("roles_delete")]
pub async fn delete_role(form: web::Form<DeleteRoleForm>, db: web::Data<DbConn>) -> Result<impl Responder, actix_web::Error> { 
    let res = form.validate();
    if let Err(ref e) = res {
        if e.is_struct_invalid("invalid_role") { 
            FlashMessage::error("Cannot Delete This Role").send();
            return Ok(see_other("/account/roles"));
        }
    }
    res.map_err(e500)?;

    let res = db::delete_role(&db, form.into_inner()).await;
    if let Err(ref e) = res {
        if e.is_foreign_key_constraint() {
            FlashMessage::error("Cannot Delete This Role - Users Still Associated").send();
            return Ok(see_other("/account/roles"));
        }
    }
    res.map_err(e500)?;

    // TODO: change flash message type for success
    FlashMessage::error("Role Deleted").send();
    Ok(see_other("/account/roles"))
}