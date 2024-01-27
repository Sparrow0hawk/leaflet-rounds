FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --bin leaflet_rounds && mv ./target/release/leaflet_rounds ./leaflet_rounds

# Runtime image
FROM debian:bookworm-slim as main

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /usr/src/app/leaflet_rounds /app/leaflet_rounds

# Run the app
CMD ./leaflet_rounds --port 8080 --host 0.0.0.0
