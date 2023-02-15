use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, Responder, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sailfish::TemplateOnce;
use sea_orm::DbConn;
use secrecy::Secret;

use crate::auth::{validate_credentials, Credentials, AuthError};
use crate::session_state::TypedSession;
use crate::utils::{see_other, error_chain_fmt, e500, ValidationErrorsExt};
use validator::{Validate, ValidationError};
// "everythinghastostartsomewhere"


#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
struct LoginPage<'a> {
    pub messages: Vec<&'a str>,
}

#[get("/login")]
pub async fn view_login(flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let body = LoginPage {
        messages: messages,
    }
    .render_once()
    .map_err(e500)?;
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))       
}

//=======================================================================

#[derive(Validate, serde::Deserialize)]
pub struct LoginForm {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed - check email and password are correct")]
    Auth(#[source] anyhow::Error),
    #[error("Validation failed - check email and password are valid")]
    Validation(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/login"))
}

#[tracing::instrument(
    name = "login",
    skip_all,
    fields(email=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[post("/login")]
pub async fn post_login (
    db: web::Data<DbConn>,
    form_data: web::Form<LoginForm>,
    session: TypedSession,
) -> Result<impl Responder, InternalError<LoginError>> {
    let form_data = form_data.into_inner();
    if let Err(e) = form_data.validate() {
        return Err(login_redirect(LoginError::Validation(e.into())));
    }
    
    let credentials = Credentials {
        email: form_data.email,
        password: form_data.password.into(),
    };

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));
    match validate_credentials(credentials, &db).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(see_other("/web/asset_items"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::Auth(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}