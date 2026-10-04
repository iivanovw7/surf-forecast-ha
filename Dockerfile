# syntax=docker/dockerfile:1

FROM rust:1.95-alpine AS builder

WORKDIR /usr/src/app

RUN apk add --no-cache \
    musl-dev \
    build-base \
    nodejs \
    npm

RUN npm install -g pnpm

RUN rustup target add \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-musl

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY rspack.config.mjs ./
COPY styles ./styles
COPY templates ./templates

RUN pnpm install --frozen-lockfile
RUN pnpm build:css

COPY Cargo.toml Cargo.lock ./
COPY src ./src

ARG TARGETARCH

RUN echo "Building for TARGETARCH=$TARGETARCH" && \
    if [ "$TARGETARCH" = "amd64" ]; then \
        cargo build --release --target x86_64-unknown-linux-musl && \
        cp target/x86_64-unknown-linux-musl/release/surf-forecast-ha /tmp/surf-forecast-ha; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        cargo build --release --target aarch64-unknown-linux-musl && \
        cp target/aarch64-unknown-linux-musl/release/surf-forecast-ha /tmp/surf-forecast-ha; \
    else \
        echo "Unsupported architecture: $TARGETARCH" && \
        exit 1; \
    fi


FROM ghcr.io/home-assistant/base:3.24

WORKDIR /app

COPY --from=builder /tmp/surf-forecast-ha /app/surf-forecast-ha
COPY --from=builder /usr/src/app/templates /app/templates
COPY --from=builder /usr/src/app/assets /app/assets
COPY config.yaml /app/config.yaml

RUN chmod +x /app/surf-forecast-ha

EXPOSE 8080

CMD ["/app/surf-forecast-ha"]
