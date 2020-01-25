use actix_web::{HttpRequest, HttpResponse, web};

use models::*;

mod internal;
mod db;
pub mod models;

pub async fn generate_token(request: web::Json<TokenRequest>) -> HttpResponse {
    let user_result = db::get_user(&request.user);
    match user_result {
        Ok(user) if user.password == request.password => {
            let my_claims = Claims {
                sub: user.name,
                exp: now_plus_days(30),
            };
            HttpResponse::Ok().json(Token { token })
        },
        Ok(user) => HttpResponse::BadRequest().body("Wrong password!"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
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
                .header("X-Auth-User", claims.sub)
                .body("")
        }
        _ => HttpResponse::Unauthorized().body("No Authorization Bearer Header")
    }
}