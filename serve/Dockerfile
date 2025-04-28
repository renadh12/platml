# Build stage
FROM rust:1.83-slim as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/serve /usr/local/bin/

# Create the model storage directory
RUN mkdir -p /app/model_storage

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8080

# Run the binary
CMD ["serve"]