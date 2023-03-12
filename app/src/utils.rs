use actix_web::rt::task::JoinHandle;
use validator::{ValidationErrors, ValidationErrorsKind};
use sea_orm::{DbErr, RuntimeErr};


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
    fn is_struct_invalid(&self, code: &str) -> bool;
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

    fn is_struct_invalid(&self, code: &str) -> bool {
        let emap = self.errors();
        if let Some(e) = emap.get("__all__") {
            match e {
                ValidationErrorsKind::Struct(_) => { },
                ValidationErrorsKind::List(_) => { },
                ValidationErrorsKind::Field(fields) => {
                    for f in fields {
                        if f.code == code {
                            return true;
                        }
                    }
                }
            }
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
            DbErr::Exec(RuntimeErr::SqlxError(error)) => {
                match error {
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
            }
            DbErr::Query(RuntimeErr::SqlxError(error)) => {
                match error {
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
            DbErr::Query(RuntimeErr::SqlxError(error)) => match error {
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

