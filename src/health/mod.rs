use actix_web::HttpResponse;

pub async fn ok() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}