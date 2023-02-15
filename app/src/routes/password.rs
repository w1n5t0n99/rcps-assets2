use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, Responder, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sailfish::TemplateOnce;
use sea_orm::DbConn;
use secrecy::{Secret, ExposeSecret};

use crate::auth::{validate_credentials, change_password, Credentials, AuthError, Client};
use crate::session_state::TypedSession;
use crate::utils::{see_other, error_chain_fmt, e500, ValidationErrorsExt};
use validator::{Validate, ValidationError};
// "everythinghastostartsomewhere"


#[derive(TemplateOnce)]
#[template(path = "password.stpl")]
struct PasswordPage<'a> {
    pub messages: Vec<&'a str>,
    pub name: String,
}

#[get("/password")]
pub async fn view_change_password(client: web::ReqData<Client>, flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();
    let name = client.into_inner().name;

    let body = PasswordPage {
        messages,
        name,
    }
    .render_once()
    .map_err(e500)?;
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))       
}

//====================================================================

#[derive(serde::Deserialize)]
pub struct PasswordForm {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

#[tracing::instrument(name = "Change Password", skip_all)]
#[post("/password")]
pub async fn post_change_password (
    db: web::Data<DbConn>,
    form_data: web::Form<PasswordForm>,
    client: web::ReqData<Client>,
) -> Result<impl Responder, actix_web::Error> {
    let form_data = form_data.into_inner();
    let client = client.into_inner();

    if form_data.new_password.expose_secret() != form_data.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/web/password"));
    }

    let credentials = Credentials {
        email: client.email,
        password: form_data.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &db).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/web/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    change_password(client.user_id, form_data.new_password, &db)
        .await
        .map_err(e500)?;

    FlashMessage::error("Your password has been changed.").send();
    Ok(see_other("/web/asset_items"))
}

