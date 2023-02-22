use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, Responder, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sailfish::TemplateOnce;
use sea_orm::DbConn;
use secrecy::Secret;

use crate::auth::{validate_credentials, Credentials, AuthError};
use crate::session_state::TypedSession;
use crate::utils::{see_other, error_chain_fmt, e500};


#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
struct LoginPage<'a> {
    pub messages: Vec<&'a str>,
    pub sign_in_url: String,
}

#[get("/sign_in")]
pub async fn view_sign_in(session: TypedSession, flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    if let Ok(Some(_)) = session.get_user_id() {
        FlashMessage::error("You are already logged in".to_string()).send();
        return Ok(see_other("/groups/asset_items"));
    }

    
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let body = LoginPage {
        messages: messages,
        sign_in_url: "/users/sign_in".to_string(),
    }
    .render_once()
    .map_err(e500)?;
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))       
}

//=======================================================================

#[derive(serde::Deserialize)]
pub struct LoginForm {
    email: String,
    password: Secret<String>,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed - check email and password are correct")]
    Auth(#[source] anyhow::Error),
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
    InternalError::from_response(e, see_other("/users/sign_in"))
}

#[tracing::instrument(
    name = "login",
    skip_all,
    fields(email=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[post("/sign_in")]
pub async fn post_sign_in (
    db: web::Data<DbConn>,
    form_data: web::Form<LoginForm>,
    session: TypedSession,
) -> Result<impl Responder, InternalError<LoginError>> {
    let form_data = form_data.into_inner();
    
    let credentials = Credentials {
        email: form_data.email,
        password: form_data.password,
    };

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));
    match validate_credentials(credentials, &db).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(see_other("/groups/asset_items"))
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