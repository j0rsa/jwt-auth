use actix_web::{App, HttpServer, web};
mod health;
mod token;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(web::resource("/health").route(web::get().to(health::ok)))
//            .service(web::resource("/auth/token").route(web::post().to(token::generate_token)))
            .service(web::resource("/auth/refresh").route(web::post().to(token::refresh)))
            .service(web::resource("/auth/check").route(web::post().to(token::check)))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}