use oso::PolarClass;
use serde::{Serialize, Deserialize};

pub mod password;
pub mod jwt;
pub mod jwt_middleware;
pub mod authorize;


#[derive(Debug, Clone, Serialize, Deserialize, PolarClass)]
pub struct ApiClient {
    #[polar(attribute)]
    pub user_id: uuid::Uuid,
    #[polar(attribute)]
    pub org_id: uuid::Uuid,
    #[polar(attribute)]
    pub role: String,
}