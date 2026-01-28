# Stage 1: Build stage
FROM rust:bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  protobuf-compiler \
  && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock build.rs ./

# Copy proto files
COPY proto/ proto/

# Copy source code
COPY src/ src/

# Build the application in release mode
RUN cargo build --release --bin gei-server

# Create directory structure that will be copied to runtime stage
RUN mkdir -p /tmp/gei-runtime/var/lib/gei && \
  mkdir -p /tmp/gei-runtime/home/gei

# Stage 2: Runtime stage
# Use distroless with nonroot user (uid 65532)
FROM gcr.io/distroless/cc-debian12:nonroot AS runner

# Copy directory structure from builder with proper ownership
COPY --from=builder --chown=65532:65532 /tmp/gei-runtime/var/lib/gei /var/lib/gei
COPY --from=builder --chown=65532:65532 /tmp/gei-runtime/home/gei /home/gei

WORKDIR /home/gei

# Copy the binary from builder with proper ownership
COPY --from=builder --chown=65532:65532 /app/target/release/gei-server /usr/local/bin/gei-server

# Set environment variables
ENV HOSTNAME=0.0.0.0
ENV DATABASE_URL=sqlite:///var/lib/gei/schedules.db

ENV RUST_LOG=info
ENV TERM=xterm-256color
ENV FORCE_COLOR=1

# Expose gRPC port
EXPOSE 50053

# Health check (note: distroless doesn't have shell, so health checks are limited)
# HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
#   CMD pgrep gei-server || exit 1

# Run the server
CMD ["gei-server"]
