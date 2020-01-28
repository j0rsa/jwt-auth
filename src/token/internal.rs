extern crate jsonwebtoken as jwt;

use std::env;
use std::ops::Add;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jwt::{decode, encode, Header, Validation};

use token::models::Claims;
use crate::token;
use uuid::Uuid;
use self::jwt::Algorithm;
use serde_json::Value;

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn now_plus_days(days: u64) -> u128 {
    SystemTime::now()
        .add(Duration::from_secs(days * 24 * 60 * 60))
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn get_env_token_secret() -> String {
    env::var("TOKEN_SECRET").expect("No token secret found!")
}

pub fn generate_token(user_id: String, user_name: String) -> String {
    generate_token_with_secret(user_id, user_name, &get_env_token_secret())
}

fn generate_token_with_secret(sub: String, name: String, secret: &String) -> String {
    let my_claims = Claims {
        iss: env_iss(),
        sub,
        iat: now(),
        exp: now_plus_days(env_exp_days()),
        aud: env_aud(),
        nbf: now_plus_days(env_nbf_days()),
        jti: Uuid::new_v4().to_string(),
        name,
    };
    encode(&Header::default(), &my_claims, secret.as_ref())
        .expect("Unable to encode claims")
}

fn env_aud() -> String {
    env::var("JWT_AUD").unwrap_or("".to_string())
}

fn env_iss() -> String {
    env::var("JWT_ISS").unwrap_or("".to_string())
}

fn env_exp_days() -> u64 {
    return match u64::from_str(
        env::var("JWT_EXP_DAYS").unwrap_or("30".to_string()).as_ref()
    ) {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
}

fn env_nbf_days() -> u64 {
    return match u64::from_str(
        env::var("JWT_NBF_DAYS").unwrap_or("0".to_string()).as_ref()
    ) {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
}

pub fn refresh_token(token: &str) -> String {
    refresh_token_with_secret(token, &get_env_token_secret())
}

fn refresh_token_with_secret(token: &str, secret: &String) -> String {
    let claims = get_claims_with_secret(token, secret);
    generate_token_with_secret(claims.sub, claims.name, secret)
}

pub fn get_claims(token: &str) -> Claims {
    get_claims_with_secret(token, &get_env_token_secret())
}

fn get_claims_with_secret(token: &str, secret: &String) -> Claims {
    let claims = decode::<Claims>(&token, secret.as_ref(), &jwt_validation())
        .expect("Unable to decode token").claims;
    claims
}

fn jwt_validation() -> Validation {
    Validation {
        leeway: env_leeway(),

        validate_exp: true,
        validate_nbf: true,

        iss: Some(env_iss()),
        sub: None,
        aud: Some(Value::String(env_aud())),

        algorithms: vec![Algorithm::HS256],
    }
}

fn env_leeway() -> i64 {
    return match i64::from_str(
        env::var("JWT_LEEWAY_SEC").unwrap_or("0".to_string()).as_ref()
    ) {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
}

const BEARER_LENGTH: usize = "Bearer ".len();

pub fn get_bearer_token(authorization_header: String) -> Option<String> {
    if authorization_header.starts_with("Bearer") {
        Option::Some(authorization_header[BEARER_LENGTH..].to_string())
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use internal::*;

    use crate::token::internal;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_bearer_token() {
        let auth = "Bearer <token>".to_string();
        let resp = get_bearer_token(auth);
        assert_eq!(resp, Option::Some("<token>".to_string()));
    }

    #[test]
    fn test_get_claims() {
        let token= "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIiLCJzdWIiOiIxMjM0NTY3ODkwIiwiYXVkIjoiIiwiZXhwIjoxNTgyODQyODUzMzEyLCJuYmYiOjE1ODAyNTA4NTMzMTIsImlhdCI6MTU4MDI1MDg1MzMxMiwianRpIjoiZDEwM2FiM2QtZmM1My00OTM2LThkZjQtM2FkNTdkNmI1YjNmIiwibmFtZSI6IjEyM3Rlc3QifQ.xa57RMHUD3sTnu561IsSedgd-j627GrrKMInQt_zATk";
        let secret = "test".to_string();
        let claims = get_claims_with_secret(&token.to_string(), &secret);
        assert_eq!(claims.sub, "1234567890");
    }

    #[test]
    fn test_generate_token_with_secret() {
        let secret = "test".to_string();
        let id = "id1".to_string();
        let name = "123test".to_string();
        let token = generate_token_with_secret(id.clone(), name.clone(), &secret);
        assert_ne!(token, "");
        let claims = get_claims_with_secret(&token, &secret);
        assert_eq!(claims.sub, id);
        assert_eq!(claims.name, name);
    }

    #[test]
    fn test_refresh_token_with_secret() {
        let secret = "test".to_string();
        let id = "id1".to_string();
        let name = "123test".to_string();
        let token = generate_token_with_secret(id.clone(), name.clone(), &secret);
        assert_ne!(token, "");
        let claims = get_claims_with_secret(&token, &secret);
        sleep(Duration::from_millis(1));
        let refreshed_token = refresh_token_with_secret(&token, &secret);
        let refreshed_claims = get_claims_with_secret(&refreshed_token, &secret);
        assert_eq!(refreshed_claims.sub, id);
        assert_eq!(refreshed_claims.name, name);
        assert!(refreshed_claims.iat > claims.iat)
    }
}
