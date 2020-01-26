use actix_web::{HttpRequest, HttpResponse, web};

use models::*;
use crate::token::db::Pool;
use std::env;

mod internal;
mod db;
mod sha;
pub mod models;

pub async fn generate_token(pool: actix_web::web::Data<Pool>, request: web::Json<TokenRequest>) -> HttpResponse {
    let user_result = db::get_user(&pool,&request.user);
    match user_result {
        Ok(user) if password_is_valid(&user.password, &request.password) => {
            let token = internal::generate_token(user.id,user.name);
            HttpResponse::Ok().json(NewTokenResponse { token })
        },
        Ok(_) => HttpResponse::BadRequest().body("Wrong password!"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

fn password_is_valid(db_password: &str, request_password: &str) -> bool {
    let check_type = env::var("PASSWORD_CHECK_TYPE").unwrap_or("RAW".to_string());
    match check_type.to_uppercase().as_ref() {
        "RAW" => db_password == request_password,
        "SHA256" => db_password == sha::sha256hash(request_password),
        "SHA512" => db_password == sha::sha512hash(request_password),
        _ => panic!("Unsupported password check type {}", check_type)
    }
}

pub async fn refresh(request: web::Json<RefreshTokenRequest>) -> HttpResponse {
    let token = internal::refresh_token(&request.token);
    let new_token = NewTokenResponse { token };
    HttpResponse::Ok().json(new_token)
}

pub async fn check(req: HttpRequest) -> HttpResponse {
    let header = req.headers().get("Authorization");
    match header {
        Some(header) => {
            let authorization_header_value = header.to_str()
                .expect("Authorization has no string value")
                .to_string();
            check_auth_value(authorization_header_value)
        }
        _ => HttpResponse::Unauthorized().body("No Authorization Header")
    }
}

fn check_auth_value(auth: String) -> HttpResponse {
    let token = internal::get_bearer_token(auth);
    match token {
        Some(bearer) => {
            let claims = internal::get_claims(&bearer);
            HttpResponse::Ok()
                .header("X-Auth-Id", claims.sub)
                .header("X-Auth-User", claims.name)
                .body("")
        }
        _ => HttpResponse::Unauthorized().body("No Authorization Bearer Header")
    }
}