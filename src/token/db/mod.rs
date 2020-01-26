use std::io;
use std::io::ErrorKind;
use r2d2_postgres::postgres::{NoTls};
use models::User;
use r2d2;
use r2d2_postgres;

pub mod models;

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>>;
pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>>;

pub fn get_user(pool: &Pool,name: &String) -> Result<User, io::Error> {
//    const USER_NAME: &'static str = &env_or("DB_QUERY_USER_NAME", "name");
//const USER_PASS: &'static str = &env_or("DB_QUERY_USER_PASSWORD", "password");
//const USERS_TABLE: &'static str = &env_or("DB_QUERY_USERS_TABLE", "users");
    let mut connection: Connection = pool.get().unwrap();
    let result = connection.query(
        "SELECT {0}, {1} FROM {2} WHERE {0}=$1",
        &[name]);
    match result {
        Ok(rows) => {
            if rows.len() == 0 { return Err(io::Error::new(ErrorKind::NotFound, "User not found")) };
            if rows.len() > 1 { return Err(io::Error::new(ErrorKind::InvalidInput, "User is not unique")) };
            let row = rows.get(0).unwrap();
            let user = User {
                name: row.get(0),
                password: row.get(1)
            };
            Ok(user)
        }
        Err(e) => Err(io::Error::new(ErrorKind::Other, e.to_string()))
    }
}
