

use chrono:: {DateTime, Duration, TimeDelta, Utc};
use jsonwebtoken:: {decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde:: {Deserialize, Serialize};
use super::constants;

#[derive (Serialize, Deserialize)]
pub struct Claims{
pub exp: usize,
pub iat: usize,
pub email: String,
pub id: i32
}

pub fn encode_jwt (email: String, id: i32) -> Result<String,jsonwebtoken::errors::Error> {
let now: DateTime<Utc> = Utc::now();
let expire: TimeDelta = Duration:: hours (24);
let claims: Claims = Claims{
    exp: (now+expire).timestamp() as usize,
iat: now.timestamp() as usize,
email,
id,
};

let secret: String = (*constants::SECRET).clone();

encode( &Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}



pub fn decade_iwt(jwt: String) -> Result<TokenData<Claims>,jsonwebtoken::errors::Error> {
    let secret: String = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
     &jwt,
     &DecodingKey::from_secret(secret.as_ref()),
     &Validation::default()
    );
    claim_data
    }




