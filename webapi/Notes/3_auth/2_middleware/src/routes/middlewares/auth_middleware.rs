use core::error;

use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header::{HeaderValue, AUTHORIZATION}, middleware::Next, Error};
use jsonwebtoken::TokenData;

use crate::utils::{api_response::{self, ApiResponse}, jwt};

// use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::{header::AUTHORIZAT
//     use actix_web_lab::middleware::Next;
    
    // use crate::utils::{api_response::{self, ApiResponse}, jwt::decode_jwt};
    
    pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth: Option<&HeaderValue> = req.headers().get( AUTHORIZATION);
    
    if auth.is_none() {
    return Err(Error::from(api_response::ApiResponse::new( 401, "Unauthorised".to_string())));
    }
    
    let token: String = auth.unwrap().to_str().unwrap().replace("bearer", "").to_owned();
    // let claim: TokenData<jwt::Claims> = jwt::decode_jwt(token).unwrap();
    
    next.call(req).await 
    .map_err(|err:  Error| Error::from(ApiResponse::new( 500,err.to_string())))
    }