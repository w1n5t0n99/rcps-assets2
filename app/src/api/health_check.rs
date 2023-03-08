use actix_web::{get, HttpResponse, Responder};


#[get("/health_check")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and Sea-ORM";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}