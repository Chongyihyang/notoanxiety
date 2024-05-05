use actix_web::http::StatusCode;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use super::constants;


#[derive(Serialize,Deserialize)]
pub struct Claims{
    pub exp: usize,
    pub iat: usize,
    pub username: String
}

#[allow(dead_code)]
pub fn encode_jwt(username: String) -> Result<String,StatusCode>{

    let now = Utc::now();
    let expire = Duration::minutes(20);

    let claim = Claims{ iat: now.timestamp() as usize, exp: (now+expire).timestamp() as usize, username: username };
    let secret = (*constants::TOKEN).clone();

    return encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });

}

#[allow(dead_code)]
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>,StatusCode> {
    let secret = (constants::TOKEN).clone();
    let res: Result<TokenData<Claims>, StatusCode> = decode(&jwt,&DecodingKey::from_secret(secret.as_ref()),&Validation::default())
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
    return res;
}