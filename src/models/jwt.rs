use serde::{Serialize, Deserialize};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, Header, EncodingKey, Validation, decode, DecodingKey, Algorithm};
use crate::SECRET_KEY;



#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(openid: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(TimeDelta::seconds(3600))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: openid.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret((SECRET_KEY).as_ref())).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0;
    validation.validate_exp = true;
    validation.validate_nbf = true;

    // 解码并验证JWT
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()), // 使用与生成时相同的密钥
        &validation,
    )?;

    Ok(token_data.claims)  // 返回解码后的 Claims
}