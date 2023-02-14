use actix_web::{get, Responder};


#[get("/asset_items")]
pub async fn get_asset_items() -> impl Responder {
    tracing::event!(tracing::Level::INFO, "###### asset route called #######");
    "Asset Items".to_string()
}