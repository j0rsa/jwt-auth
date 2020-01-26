#--- Build
FROM rust:stretch as cargo-build

RUN apt-get update --no-install-recommends && \
    apt-get install musl-tools --no-install-recommends -y && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.toml
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/jwt_auth*

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

#--- Run
FROM scratch

LABEL authors="red.avtovo@gmail.com, keyridan@gmail.com"

WORKDIR /app
COPY --from=cargo-build /usr/src/app/target/x86_64-unknown-linux-musl/release/jwt-auth .

EXPOSE 8080
ENV RUST_LOG=info

CMD ["./jwt-auth"]