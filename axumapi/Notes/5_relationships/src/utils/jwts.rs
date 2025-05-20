use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use migration::token;
use serde::{Deserialize, Serialize};

use super::constants;



#[derive(Serialize , Deserialize)]
pub struct Claims{
pub iat : usize , 
pub exp : usize , 
pub email : String 
}



pub fn encode_jwt (email: String) -> Result<String , StatusCode> {

    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        email: email,
    };

    let secret = (*constants::TOKEN).clone();
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR })
}


pub fn decode_jwt (token: String ) -> Result<TokenData<Claims>, StatusCode>  {  
  let secret = (*constants::TOKEN).clone();
   let res: Result<TokenData<Claims>,StatusCode> = decode(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
  
   return res;

}