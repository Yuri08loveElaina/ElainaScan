# =============================
# Stage 1: Build ElainaScan
# =============================
FROM rust:1.77-alpine AS builder

# Install dependencies for building
RUN apk add --no-cache musl-dev openssl-dev pkgconf bash

# Set workdir
WORKDIR /build

# Copy manifests first for cache
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build in release mode
RUN cargo build --release

# =============================
# Stage 2: Minimal runtime image
# =============================
FROM alpine:latest

# Install runtime dependencies and nmap
RUN apk add --no-cache libgcc libstdc++ nmap

# Create non-root user for security
RUN addgroup -S elaina && adduser -S elaina -G elaina

# Create app directory
WORKDIR /app

# Copy built binary
COPY --from=builder /build/target/release/elainascan /usr/local/bin/elainascan

# Change permissions
RUN chmod +x /usr/local/bin/elainascan

# Switch to non-root user
USER elaina

# Default command
ENTRYPOINT ["elainascan"]
