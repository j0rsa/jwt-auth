# JWT Authentication service

This is a simple service to authenticate users and check their authentication information according to JWT validity (signature, expiration, sub) 

![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/j0rsa/jwt-auth)
![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/j0rsa/jwt-auth)

![Base Docker image scratch](https://img.shields.io/badge/Base%20Image-Scratch-blue)
![Image Size](https://img.shields.io/badge/image%20size-9.8MB-green)
![MicroBadger Layers](https://img.shields.io/microbadger/layers/j0rsa/jwt-auth)

[![CodeFactor](https://www.codefactor.io/repository/github/j0rsa/jwt-auth/badge/master)](https://www.codefactor.io/repository/github/j0rsa/jwt-auth/overview/master)

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
| TOKEN_SECRET | -- | JWT HS256 Secret Key |

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