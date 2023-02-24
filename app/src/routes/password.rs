use actix_web::http::header::ContentType;
use actix_web::{get, post, Responder, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sailfish::TemplateOnce;
use sea_orm::DbConn;
use validator::Validate;

use crate::auth::{validate_credentials, change_password, Credentials, AuthError, Client};
use crate::utils::{see_other, e500, e400, ValidationErrorsExt};
use crate::domain::password_form::PasswordForm;
// "everythinghastostartsomewhere"


#[derive(TemplateOnce)]
#[template(path = "password.stpl")]
struct PasswordPage<'a> {
    pub messages: Vec<&'a str>,
    pub name: String,
    pub id: String,
}

#[get("/edit_password")]
pub async fn view_edit_password(
    client: web::ReqData<Client>,
    path: web::Path<(uuid::Uuid,)>,
    flash_messages: IncomingFlashMessages,
) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();
    let client = client.into_inner();
    let path_id = path.into_inner().0;

    if path_id != client.user_id {
        return Err(e400("page not found".to_string()));
    }

    let body = PasswordPage {
        messages,
        name: client.name,
        id: client.user_id.to_string(),
    }
    .render_once()
    .map_err(e500)?;
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))       
}

//====================================================================

#[tracing::instrument(name = "Change Password", skip_all)]
#[post("/edit_password")]
pub async fn post_edit_password (
    db: web::Data<DbConn>,
    form_data: web::Form<PasswordForm>,
    client: web::ReqData<Client>,
) -> Result<impl Responder, actix_web::Error> {
    let client = client.into_inner();
    let url = client.url_to("edit_password");
    let form_data = form_data.into_inner();

    let res = form_data.validate();
    if let Err(e) = res {
        if e.is_struct_invalid("password_length") {
            FlashMessage::error("The password must be atleast 8 characters long.").send();
            return Ok(see_other(&url));
        }
        else if e.is_struct_invalid("password_match") {
            FlashMessage::error("You entered two different new passwords - the field values must match.").send();
            return Ok(see_other(&url));
        }
    }

    let credentials = Credentials {
        email: client.email,
        password: form_data.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &db).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other(&url))
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    change_password(client.user_id, form_data.new_password, &db)
        .await
        .map_err(e500)?;

    FlashMessage::error("Your password has been changed.").send();
    Ok(see_other("/groups/asset_items"))
}

