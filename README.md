# JWT Authentication service

This is a simple service to authenticate users and check their authentication information according to JWT validity (signature, expiration, sub) 

Base Docker image: `scratch`

[![](https://images.microbadger.com/badges/image/j0rsa/jwt-auth.svg)](https://microbadger.com/images/j0rsa/jwt-auth "Get your own image badge on microbadger.com")
[![](https://images.microbadger.com/badges/version/j0rsa/jwt-auth.svg)](https://microbadger.com/images/j0rsa/jwt-auth "Get your own version badge on microbadger.com")

## Endpoints
| Method | URL | Description |
| ------:| --- | ----------- |
| `GET` | `/health` | Healthcheck  which returns Code 200|
| `POST` | `/auth/token` | Get JWT token by passing user credentials `{ "user": "name", "password": "secret"}` |
| `POST` | `/auth/refresh` | Refresh token with a new one by passing the old valid one `{ "token": "eyJhbGciOiJIUz..." }` |
| `POST` | `/auth/check` | Checks the token and returns code 200 with Header: `X-Auth-User` with user name |

## Environment variables
| Variable | Default value | Description |
| ------| --- | ----------- |
| RUST_LOG | info | defines the log level of app |
| DB_HOST | localhost | Postgres host |
| DB_PORT | 5432 | Postgres port |
| DB_USER | postgres | Postgres user |
| DB_PASSWORD | postgres | Postgres password |
| DB_NAME | postgres | Database name |
| DB_QUERY_USER_NAME | name | `Username` column name in users table |
| DB_QUERY_USER_PASSWORD | password | `Password` column name in users table |
| DB_QUERY_USERS_TABLE | users | users table |

# Build

## Build release locally
    cargo build --release

## Build release in docker and prepare an image
    docker build -t jwt-auth .
    
ref: https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

ref: https://medium.com/@gdiener/how-to-build-a-smaller-docker-image-76779e18d48a

# Troubleshooting

## Inspect image filesystem
    docker run --rm -it <image name or id> sh
## Test run
    docker run --rm -it jwt-auth