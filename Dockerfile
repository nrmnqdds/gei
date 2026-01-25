# Stage 1: Build stage
FROM rust:bookworm as builder

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

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
  && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1001 -s /bin/bash gei

# Create directory for database
RUN mkdir -p /var/lib/gei && chown gei:gei /var/lib/gei

# Switch to non-root user
USER gei
WORKDIR /home/gei

# Copy the binary from builder
COPY --from=builder /app/target/release/gei-server /usr/local/bin/gei-server

# Set environment variables
ENV DATABASE_URL=sqlite:///var/lib/gei/schedules.db
ENV RUST_LOG=info

# Expose gRPC port
EXPOSE 50051

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD pgrep gei-server || exit 1

# Run the server
CMD ["gei-server"]
