use std::{fmt::format, process::id};

use axum::{body::Body, http::{header::AUTHORIZATION, Request, StatusCode}, middleware::Next, response::Response};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use super::{api_errors::ApiError, jwts::decode_jwt};
use sea_orm::ColumnTrait;


pub async fn guard(mut req: Request<Body> , next: Next) -> Result<Response , ApiError> {


       let token = req.headers()
        .get(AUTHORIZATION)

        .ok_or(ApiError{message: "Auth error".to_owned(), status_code:StatusCode::UNAUTHORIZED , error_code:Some(48)})?
        .to_str()
        .map_err(|_| ApiError{message: "Unable to convert format".to_owned(), status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(44)})?
        .strip_prefix("Bearer")
        .ok_or(ApiError{message: "Error getting token".to_owned(), status_code:StatusCode::INTERNAL_SERVER_ERROR , error_code:Some(34)})?;

    let claims = decode_jwt(format!("{}",token)).map_err(|_|ApiError{
        status_code:StatusCode::BAD_REQUEST , message: "Can't resolve claims".to_string() , error_code: Some(55)
    })?.claims ;

    let db = req.extensions().get::<DatabaseConnection>().ok_or(
        ApiError{status_code:StatusCode::INTERNAL_SERVER_ERROR , message: "Failed to connect to database ".to_string() , error_code: Some(66)}
    )?;

    let identity = entity::user::Entity::find().filter(
        entity::user::Column::Email.eq(claims.email.to_lowercase())
    ).one(db).await
    .map_err(|error| ApiError{message: error.to_string() , error_code: Some(44) , status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(ApiError{message: "Failed to get user".to_string() , error_code: Some(44) , status_code: StatusCode::INTERNAL_SERVER_ERROR} )?;

    
    req.extensions_mut().insert(identity);


    Ok(next.run(req).await)

}