use actix_web::rt::task::JoinHandle;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use validator::ValidationErrors;
use sea_orm::{DbErr, RuntimeErr};

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

// Return an opaque 404 while preserving the error root's cause for logging.
pub fn e404<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorNotFound(e)
}

// Return an opaque 403 while preserving the error root's cause for logging.
pub fn e403<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorForbidden(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    actix_web::rt::task::spawn_blocking(move || current_span.in_scope(f))
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub trait ValidationErrorsExt {
    fn is_field_invalid(&self, field: &str) -> bool;
}

impl ValidationErrorsExt for ValidationErrors
{
    fn is_field_invalid(&self, field: &str) -> bool {
        let emap = self.errors();
        if let Some(_e) = emap.get(field) {
            return true;
        }

        false
    }
}

pub trait DbErrbExt {
    fn is_unique_key_constraint(&self) -> bool;
    fn is_foreign_key_constraint(&self) -> bool;
}

impl DbErrbExt for DbErr {
    fn is_unique_key_constraint(&self) -> bool {
        const SQLITE_CODE: &'static str = "2067";
        const POSTGRES_CODE: &'static str = "23505";

        match self {
            DbErr::Exec(RuntimeErr::SqlxError(error)) => match error {
                sqlx::Error::Database(e) => {
                    if let Some(code) = e.code() {
                        if code == SQLITE_CODE || code == POSTGRES_CODE {
                            return true;
                        }
                    }
                    false
                }
                _ => false,
            } 
            _ => false,
        }
    }

    fn is_foreign_key_constraint(&self) -> bool {
        const SQLITE_CODE: &'static str = "787";
        const POSTGRES_CODE: &'static str = "23503";

        match self {
            DbErr::Exec(RuntimeErr::SqlxError(error)) => match error {
                sqlx::Error::Database(e) => {
                    if let Some(code) = e.code() {
                        if code == SQLITE_CODE || code == POSTGRES_CODE {
                            return true;
                        }
                    }
                    false
                }
                _ => false,
            } 
            _ => false,
        }
    }
}

