# JWT Authentication service

This is a simple service to authenticate users and check their authentication information according to JWT validity (signature, expiration, sub) 

[![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/j0rsa/jwt-auth)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/j0rsa/jwt-auth)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)

[![Base Docker image scratch](https://img.shields.io/badge/Base%20Image-Scratch-blue)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)
[![Image Size](https://img.shields.io/badge/image%20size-9.71MB-green)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)
[![MicroBadger Layers](https://img.shields.io/microbadger/layers/j0rsa/jwt-auth)](https://hub.docker.com/repository/docker/j0rsa/jwt-auth)

[![CodeFactor](https://www.codefactor.io/repository/github/j0rsa/jwt-auth/badge/master)](https://www.codefactor.io/repository/github/j0rsa/jwt-auth/overview/master)

## Endpoints
| Method | URL | Description |
| ------:| --- | ----------- |
| `GET` | `/health` | Healthcheck  which returns Code 200|
| `POST` | `/auth/token` | Get JWT token by passing user credentials `{ "user": "name", "password": "secret"}` |
| `POST` | `/auth/refresh` | Refresh token with a new one by passing the old valid one `{ "token": "eyJhbGciOiJIUz..." }` |
| `POST` | `/auth/check` | Checks the token and returns code 200 with Headers: `X-Auth-Id` with user id and `X-Auth-User` with user name |

## Environment variables
| Variable | Default value | Description |
| ------| --- | ----------- |
| RUST_LOG | info | defines the log level of app |
| DB_HOST | localhost | Postgres host |
| DB_PORT | 5432 | Postgres port |
| DB_USER | postgres | Postgres user |
| DB_PASSWORD | postgres | Postgres password |
| DB_NAME | postgres | Database name |
| DB_QUERY_USER_ID | id | `Id` column name in users table |
| DB_QUERY_USER_NAME | name | `Username` column name in users table |
| DB_QUERY_USER_PASSWORD | password | `Password` column name in users table |
| DB_QUERY_USERS_TABLE | users | users table |
| TOKEN_SECRET | -- | JWT HS256 Secret Key |
| BIND_ADDRESS | 0.0.0.0 | Address of web server to listen connections |
| BIND_PORT | 8080 | Port of web server to listen connections |
| PASSWORD_CHECK_TYPE | RAW | Type to compare passwords with the one from DB (values: RAW, SHA256, SHA512, BCRYPT) |
| JWT_ISS | "" | iss (issuer): Issuer of the JWT |
| JWT_AUD | "" | aud (audience): Recipient for which the JWT is intended |
| JWT_EXP_DAYS | 30 | exp (expiration time): Time in days after which the JWT expires |
| JWT_NBF_DAYS | 0 | nbf (not before time): Time in days before which the JWT must not be accepted for processing |

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