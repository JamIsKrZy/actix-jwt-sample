
use actix_web::cookie::time::Duration;

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    email: String,
    exp: usize
}


impl Claims{
    pub fn new(email: String, exp: usize) -> Self{
        Self { 
            email, 
            exp 
        }
    }

    
}

pub fn create_claims(
    email: String, 
    header: &Header
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now()
        .checked_add_signed(TimeDelta::hours(3))
        .expect("Time is out of range")
        .timestamp() as usize;

    let claim = Claims::new(email, exp);

    println!("⚠️ Encoding Key is not safe!");
    encode(
        header, 
        &claim, 
        &EncodingKey::from_secret("sample".as_ref())
    )
}

pub fn verify_jwt(token: &str, validation: &Validation) -> Option<Claims>{
    println!("⚠️ Encoding Key is not safe!");
    decode::<Claims>(
        token, 
        &DecodingKey::from_secret("sample".as_ref()), 
        validation
    )
    .map(|v| v.claims)
    .ok()
} 