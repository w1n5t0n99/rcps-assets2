use actix_web::http::header::ContentType;
use actix_web::{get, post, Responder, web, HttpResponse, HttpRequest};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sailfish::TemplateOnce;
use sea_orm::DbConn;
use secrecy::{Secret, ExposeSecret};

use crate::auth::{validate_credentials, change_password, Credentials, AuthError, Client};
use crate::utils::{see_other, e500, e400};
use validator::{validate_must_match, validate_length};
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

#[derive(serde::Deserialize)]
pub struct PasswordForm {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

#[tracing::instrument(name = "Change Password", skip_all)]
#[post("/edit_password")]
pub async fn post_edit_password (
    db: web::Data<DbConn>,
    form_data: web::Form<PasswordForm>,
    client: web::ReqData<Client>,
) -> Result<impl Responder, actix_web::Error> {
    let form_data = form_data.into_inner();
    let client = client.into_inner();
    let url = client.url_to("edit_password");

    if validate_must_match(form_data.new_password.expose_secret(), form_data.new_password_check.expose_secret()) == false {
        FlashMessage::error("You entered two different new passwords - the field values must match.")
            .send();

        return Ok(see_other(&url));
    }

    if validate_length(form_data.new_password.expose_secret(), Some(8), None, None) == false {
        FlashMessage::error("The password must be atleast 8 characters long.")
            .send();

        return Ok(see_other(&url));
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

