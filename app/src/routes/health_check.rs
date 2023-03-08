use actix_web::{get, HttpResponse};


#[get("/health_check")]
pub async fn health_checker() -> HttpResponse {
    HttpResponse::Ok().finish()
}