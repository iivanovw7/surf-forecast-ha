# Stage 1: Build stage
FROM rust:1.80-slim AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release

# Stage 2: Runtime stage
FROM ghcr.io/home-assistant/aarch64-base:latest

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/surf_backend /app/surf_backend
COPY templates /app/templates

EXPOSE 8080

CMD [ "/app/surf_backend" ]
