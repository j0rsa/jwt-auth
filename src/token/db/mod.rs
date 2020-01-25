extern crate postgres;

use std::{env, io};
use std::io::ErrorKind;

use postgres::{Client, Error, NoTls};

use models::User;

pub mod models;

const HOST: String = env_or("DB_HOST", "localhost");
const PORT: String = env_or("DB_PORT", "5432");
const USER: String = env_or("DB_USER", "postgres");
const PASSWORD: String = env_or("DB_PASSWORD", "postgres");
const DB_NAME: String = env_or("DB_NAME", "postgres");

const USER_NAME: String = env_or("DB_QUERY_USER_NAME", "name");
const USER_PASS: String = env_or("DB_QUERY_USER_PASSWORD", "password");
const USERS_TABLE: String = env_or("DB_QUERY_USERS_TABLE", "users");

const CONNECTION: Client = Client::connect(
    &format!("host={} port={} user={} password={} dbname={}", HOST, PORT, USER, PASSWORD, DB_NAME),
    NoTls)?;

pub fn get_user(name: &String) -> Result<User, io::Error> {
    let result = CONNECTION.query(
        &format!("SELECT {0}, {1} FROM {2} WHERE {0}=$1", USER_NAME, USER_PASS, USERS_TABLE),
        &[name]);
    match result {
        Ok(rows) => {
            if rows.len() == 0 { Err(io::Error::new(ErrorKind::NotFound, "User not found")) }
            if rows.len() > 1 { Err(io::Error::new(ErrorKind::InvalidInput, "User is not unique")) }
            let row = rows.get(0)?;
            let user = User {
                name: row.get(0),
                password: row.get(1)
            };
            Ok(user)
        }
        Err(e) => Err(io::Error::new(ErrorKind::Other, e.to_string()))
    }
}

fn env_or(var: &str, default: &str) -> String {
    env::var(var).unwrap_or(default.to_string())
}