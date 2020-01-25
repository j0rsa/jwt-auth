extern crate jsonwebtoken as jwt;
use jwt::{encode, decode, Header, Validation};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::ops::Add;
use std::env;

use crate::token;
use token::models::Claims;

fn now_plus_days(days: u64) -> u128 {
    SystemTime::now()
        .add(Duration::from_secs(days * 24 * 60 * 60))
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn get_token_secret() -> String {
    env::var("TOKEN_SECRET").expect("No token secret found!")
}

pub fn generate_token(user_name: String) -> String {
    let my_claims = Claims {
        sub: user_name,
        exp: now_plus_days(30)
    };
    encode(&Header::default(), &my_claims, get_token_secret().as_ref())
        .expect("Unable to encode claims")
}

pub fn refresh_token(token: &String) -> String {
    let claims = get_claims(token);
    generate_token(claims.sub)
}

pub fn get_claims(token: &String) -> Claims {
    let claims = decode::<Claims>(&token, get_token_secret().as_ref(), &Validation::default())
        .expect("Unable to decode token").claims;
    claims
}

const BEARER_LENGTH: usize = "Bearer ".len();
pub fn get_bearer_token(authorization_header: String) -> Option<String>{
    if authorization_header.starts_with("Bearer") {
        Option::Some(authorization_header[BEARER_LENGTH..].to_string())
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use crate::token::internal;
    use internal::*;

    #[test]
    fn test_bearer_token() {
        let auth = "Bearer <token>".to_string();
        let resp = get_bearer_token(auth);
        assert_eq!(resp, Option::Some("<token>"));
    }
}
