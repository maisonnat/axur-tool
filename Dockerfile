# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release -p axur-backend

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/axur-backend /app/axur-backend

# Expose port
EXPOSE 3001

# Set environment
ENV RUST_LOG=axur_backend=info,tower_http=info

# Run the binary
CMD ["./axur-backend"]
