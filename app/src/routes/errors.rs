use actix_web::body::{EitherBody, BoxBody};
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, Result};
use reqwest::header::HeaderValue;
use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
struct ErrorPage<'a> {
    pub header: &'a str,
    pub description: &'a str,
    pub msg: &'a str,
}

fn handle_errors<B>(page: ErrorPage, res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    
    let body = page.render_once().unwrap_or("Error".to_string());
    let bbody = BoxBody::new(body);
    
    let mut res: ServiceResponse<EitherBody<B>> =
        res.map_body(|_, _| EitherBody::<B, BoxBody>::right(bbody));

     // Headers must be manually set because Actix-Web renders no content by default.
     let headers = res.response_mut().headers_mut();
     // Web document
     headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
     // Proxies (Cloudflare) love to cache error pages permanently. Explicitly say not to do that.
     headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
     headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
     
    Ok(ErrorHandlerResponse::Response(res))
}

pub fn render_400<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let body = ErrorPage {
        header: "ERROR 400",
        description: "Bad Request",
        msg: " ",     
    };

    handle_errors::<B>(body, res)
}

pub fn render_403<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let body = ErrorPage {
        header: "ERROR 403",
        description: "Forbidden",
        msg: " ",     
    };

    handle_errors::<B>(body, res)
}

pub fn render_404<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let body = ErrorPage {
        header: "ERROR 404",
        description: "Not Found",
        msg: " ",     
    };

    handle_errors::<B>(body, res)
}

pub fn render_500<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let body = ErrorPage {
        header: "ERROR 500",
        description: "Internal Server Error",
        msg: " ",     
    };

    handle_errors::<B>(body, res)
}