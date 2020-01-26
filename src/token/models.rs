use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    // issued at
    pub iat: u128,
    // expired after
    pub exp: u128,
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
