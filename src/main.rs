use actix_web::{App, HttpServer, web};
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::{NoTls};
mod health;
mod token;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client_url = &format!("host={} port={} user={} password={} dbname={}",
                              option_env!("DB_HOST").unwrap_or("localhost"),
                              option_env!("DB_PORT").unwrap_or("5432"),
                              option_env!("DB_USER").unwrap_or("postgres"),
                              option_env!("DB_PASSWORD").unwrap_or("postgres"),
                              option_env!("DB_NAME").unwrap_or("postgres"));

    let manager = PostgresConnectionManager::new(
        client_url.parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new( move ||
        App::new()
            .data(pool.clone())
            .service(web::resource("/health").route(web::get().to(health::ok)))
            .service(web::resource("/auth/token").route(web::post().to(token::generate_token)))
            .service(web::resource("/auth/refresh").route(web::post().to(token::refresh)))
            .service(web::resource("/auth/check").route(web::post().to(token::check)))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}