use actix_web::HttpResponse;
use reqwest::StatusCode;
use serde::Serialize;


/*
400 Bad Request – This means that client-side input fails validation.
401 Unauthorized – This means the user isn’t not authorized to access a resource. It usually returns when the user isn’t authenticated.
403 Forbidden – This means the user is authenticated, but it’s not allowed to access a resource.
404 Not Found – This indicates that a resource is not found.
500 Internal server error – This is a generic server error. It probably shouldn’t be thrown explicitly.
502 Bad Gateway – This indicates an invalid response from an upstream server.
503 Service Unavailable – This indicates that something unexpected happened on server side (It can be anything like server overload, some parts of the system failed, etc.).
 */

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