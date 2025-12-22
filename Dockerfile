# Runtime-only image (no compilation)
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy pre-built binary from GitHub Actions
COPY axur-backend /app/axur-backend

# Make executable
RUN chmod +x /app/axur-backend

# Expose port
EXPOSE 3001

# Set environment
ENV RUST_LOG=axur_backend=info,tower_http=info

# Run the binary
CMD ["./axur-backend"]
