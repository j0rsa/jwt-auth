#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::{web, App, HttpServer};
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::env;

mod health;
mod token;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let client_url = &format!(
        "host={} port={} user={} password={} dbname={}",
        env::var("DB_HOST").unwrap_or("localhost".to_string()),
        env::var("DB_PORT").unwrap_or("5432".to_string()),
        env::var("DB_USER").unwrap_or("postgres".to_string()),
        env::var("DB_PASSWORD").unwrap_or("postgres".to_string()),
        env::var("DB_NAME").unwrap_or("postgres".to_string())
    );
    let address = env::var("BIND_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("BIND_PORT").unwrap_or("8080".to_string());

    info!("Connecting to DB: {}", &client_url);
    let manager = PostgresConnectionManager::new(client_url.parse().unwrap(), NoTls);
    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/health").route(web::get().to(health::ok)))
            .service(web::resource("/auth/token").route(web::post().to(token::generate_token)))
            .service(web::resource("/auth/refresh").route(web::post().to(token::refresh)))
            .service(web::resource("/auth/check").route(web::post().to(token::check)))
    })
        .bind(format!("{}:{}", &address, &port))?
        .run()
        .await
}
