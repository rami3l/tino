# HACK: Version pinned due to https://github.com/LukeMathWalker/cargo-chef/issues/290
FROM lukemathwalker/cargo-chef:0.1.68-rust-latest AS chef
# FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef

# ===== Plan Stage =====
FROM chef AS tino-planner
ENV APP_NAME=tino
WORKDIR /app/${APP_NAME}
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ===== Build Stage =====
FROM chef AS tino-builder
ENV APP_NAME=tino
WORKDIR /app/${APP_NAME}
COPY --from=tino-planner /app/${APP_NAME}/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# This is not needed as it's included in `chef` already.
# RUN apk add --no-cache musl-dev
RUN cargo install --path .

# ===== Run Stage =====
FROM alpine:3.15 AS tino
ENV APP_NAME=tino

# Copy only required data into this image
COPY --from=tino-builder /usr/local/cargo/bin/$APP_NAME .

# Expose application port
# EXPOSE 8081

# Start app
SHELL ["/bin/sh", "-c"]
ENTRYPOINT ./$APP_NAME
