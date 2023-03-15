use actix_web::HttpResponse;
use reqwest::StatusCode;
use serde::Serialize;


#[derive(Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}


// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T, S>(status: S, message: S, e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
    S: Into<String>,
{
    let error_response = ErrorResponse {
        status: status.into(),
        message: message.into(),
    };

    let response = HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error_response);

    actix_web::error::InternalError::from_response(e, response).into()
}

// CONFLICT
pub fn e409<T, S>(status: S, message: S, e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
    S: Into<String>,
{
    let error_response = ErrorResponse {
        status: status.into(),
        message: message.into(),
    };

    let response = HttpResponse::build(StatusCode::CONFLICT).json(error_response);

    actix_web::error::InternalError::from_response(e, response).into()
}

// BAD_REQUEST
pub fn e400<T, S>(status: S, message: S, e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
    S: Into<String>,
{
    let error_response = ErrorResponse {
        status: status.into(),
        message: message.into(),
    };

    let response = HttpResponse::build(StatusCode::BAD_REQUEST).json(error_response);

    actix_web::error::InternalError::from_response(e, response).into()
}

// UNAUTHORIZED
pub fn e401<T, S>(status: S, message: S, e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
    S: Into<String>,
{
    let error_response = ErrorResponse {
        status: status.into(),
        message: message.into(),
    };

    let response = HttpResponse::build(StatusCode::UNAUTHORIZED).json(error_response);

    actix_web::error::InternalError::from_response(e, response).into()
}

// FORBIDDEN
pub fn e403<T, S>(status: S, message: S, e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
    S: Into<String>,
{
    let error_response = ErrorResponse {
        status: status.into(),
        message: message.into(),
    };

    let response = HttpResponse::build(StatusCode::FORBIDDEN).json(error_response);

    actix_web::error::InternalError::from_response(e, response).into()
}

// NOT_FOUND
pub fn e404<T, S>(status: S, message: S, e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
    S: Into<String>,
{
    let error_response = ErrorResponse {
        status: status.into(),
        message: message.into(),
    };

    let response = HttpResponse::build(StatusCode::NOT_FOUND).json(error_response);

    actix_web::error::InternalError::from_response(e, response).into()
}

