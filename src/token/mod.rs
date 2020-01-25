use actix_web::{HttpResponse, web, HttpRequest};

mod internal;
pub mod models;

use models::*;

//async fn generate_token(request: web::Json<TokenRequest>) -> HttpResponse {
//
//    let my_claims = Claims {
//        sub,
//        exp: now_plus_days(30)
//    };
//    let token = encode(&Header::default(), &my_claims, "secret".as_ref())
//        .expect("Unable to encode claims");
//    HttpResponse::Ok().json(Token {
//        token
//    })
//}

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