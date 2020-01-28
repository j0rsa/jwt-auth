use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    // expired after
    pub exp: u128,
    // valid not before
    pub nbf: u128,
    // issued at
    pub iat: u128,
    // jwt id
    pub jti: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTokenResponse {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    pub user: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub token: String
}
