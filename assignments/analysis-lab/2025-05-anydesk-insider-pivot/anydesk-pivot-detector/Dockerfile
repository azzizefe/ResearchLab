# 12.1.1: Multi-stage build olustur (builder + runtime)
# Stage 1: Builder (Rust builder image)
FROM rust:1.78-slim AS builder

WORKDIR /usr/src/app

# Build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 12.1.2: Rust builder image
RUN cargo build --release

# Stage 2: Runtime (Runtime image - debian:bookworm-slim)
FROM debian:bookworm-slim

WORKDIR /app

# Runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 12.1.4: Non-root kullanici ile calistir
RUN groupadd -r anydesk && useradd -r -g anydesk anydesk-user
RUN mkdir -p /app/reports && chown -R anydesk-user:anydesk /app/reports
USER anydesk-user

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/anydesk-pivot-detector /app/

# Environment defaults
ENV APP_REPORT_OUTPUT_DIR=/app/reports
ENV ANYDESK_TRACE_PATH=/var/log/anydesk/ad.trace

# 12.1.5: Health check endpoint/komut ekle
HEALTHCHECK --interval=60s --timeout=5s --start-period=5s --retries=3 \
  CMD /app/anydesk-pivot-detector --version || exit 1

# Default execution
ENTRYPOINT ["/app/anydesk-pivot-detector"]
CMD ["monitor"]
