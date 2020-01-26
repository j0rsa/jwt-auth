extern crate postgres;

use std::io;
use std::io::ErrorKind;

use postgres::{Client, NoTls};

use models::User;

pub mod models;

pub fn get_user(name: &String) -> Result<User, io::Error> {
    let client_url = &format!("host={} port={} user={} password={} dbname={}",
                                            option_env!("DB_HOST").unwrap_or("localhost"),
                                            option_env!("DB_PORT").unwrap_or("5432"),
                                            option_env!("DB_USER").unwrap_or("postgres"),
                                            option_env!("DB_PASSWORD").unwrap_or("postgres"),
                                            option_env!("DB_NAME").unwrap_or("postgres"));
    let mut connection = Client::connect(client_url, NoTls).unwrap();
//    const USER_NAME: &'static str = &env_or("DB_QUERY_USER_NAME", "name");
//const USER_PASS: &'static str = &env_or("DB_QUERY_USER_PASSWORD", "password");
//const USERS_TABLE: &'static str = &env_or("DB_QUERY_USERS_TABLE", "users");

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
