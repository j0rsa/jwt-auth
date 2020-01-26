use std::{io, env};
use std::io::ErrorKind;

use r2d2;
use r2d2_postgres;
use r2d2_postgres::postgres::NoTls;

use models::User;

pub mod models;

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>>;
pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>>;

pub fn get_user(pool: &Pool, name: &String) -> Result<User, io::Error> {
    let mut connection: Connection = pool.get().unwrap();
    let query = &find_user() as &str;
    let result = connection.query(query, &[name]);
    match result {
        Ok(rows) => {
            if rows.len() == 0 { return Err(io::Error::new(ErrorKind::NotFound, "User not found")) };
            if rows.len() > 1 { return Err(io::Error::new(ErrorKind::InvalidInput, "User is not unique")) };
            let row = rows.get(0).unwrap();
            let user = User {
                name: row.get(0),
                password: row.get(1),
            };
            Ok(user)
        }
        Err(e) => Err(io::Error::new(ErrorKind::Other, e.to_string()))
    }
}

fn find_user() -> String {
    format!("SELECT {0}, {1} FROM {2} WHERE {0}=$1",
            env::var("DB_QUERY_USER_NAME").unwrap_or("name".to_string()),
            env::var("DB_QUERY_USER_PASSWORD").unwrap_or("password".to_string()),
            env::var("DB_QUERY_USERS_TABLE").unwrap_or("users".to_string()),
    )
}

#[cfg(test)]
mod tests {
    use crate::token::db::{find_user, get_user};
    use r2d2_postgres::PostgresConnectionManager;
    use r2d2_postgres::postgres::{NoTls};
    use std::io::ErrorKind;

    #[test]
    fn test_find_user() {
        let query = find_user();
        assert_eq!("SELECT name, password FROM users WHERE name=$1", query);
    }

    #[test]
    fn test_get_user_one() {
        let manager = PostgresConnectionManager::new(
            "host=localhost user=postgres password=postgres dbname=postgres".parse().unwrap(),
            NoTls,
        );
        let pool = r2d2::Pool::new(manager).unwrap();
        let username = &"user".to_string();
        let result = get_user(&pool, username);
        assert_eq!("user", result.unwrap().name)
    }

    #[test]
    fn test_get_user_double() {
        let manager = PostgresConnectionManager::new(
            "host=localhost user=postgres password=postgres dbname=postgres".parse().unwrap(),
            NoTls,
        );
        let pool = r2d2::Pool::new(manager).unwrap();
        let username = &"double".to_string();
        let result = get_user(&pool, username);
        assert_eq!(ErrorKind::InvalidInput, result.err().unwrap().kind())
    }

    #[test]
    fn test_get_user_none() {
        let manager = PostgresConnectionManager::new(
            "host=localhost user=postgres password=postgres dbname=postgres".parse().unwrap(),
            NoTls,
        );
        let pool = r2d2::Pool::new(manager).unwrap();
        let username = &"none".to_string();
        let result = get_user(&pool, username);
        assert_eq!(ErrorKind::NotFound, result.err().unwrap().kind())
    }
}
