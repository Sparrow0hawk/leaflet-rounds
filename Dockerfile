FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --bin actix_starter && mv ./target/release/actix_starter ./actix_starter

# Runtime image
FROM debian:bookworm-slim as main

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/actix_starter /app/actix_starter

# Run the app
CMD ./actix_starter --port 8080 --host 0.0.0.0
