use actix_web::{http, web, HttpMessage};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_web_lab::middleware::Next;

use crate::auth::jwt::TokenClaims;
use crate::error_responses::{e500, e401};

use super::JwtData;


pub async fn reject_invalid_jwt(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {

    let decoding_key = req.app_data::<web::Data<DecodingKey>>()
        .ok_or_else(|| e500("error", "Unexpected server error occured", "AuthError"))?;

    let token = req
        .cookie("token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get(http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
        })
        .ok_or_else(|| e401("fail", "You are not logged in, please provide token", "AuthError"))?;
    
    let claims = decode::<TokenClaims>(
            &token,
            &decoding_key,
            &Validation::default(),
        )
        .map_err(|e| e401("fail", "Invalid token", e))
        .map(|t| t.claims)?;

    // TODO: insert client struct instead of plain uuid
    let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).map_err(|e| e500("error", "Unexpected server error occured", e))?;
    let org_id = uuid::Uuid::parse_str(claims.org.as_str()).map_err(|e| e500("error", "Unexpected server error occured", e))?;

    let jwt_data = JwtData { user_id, org_id };
    req.extensions_mut().insert::<JwtData>(jwt_data);

    next.call(req).await
}