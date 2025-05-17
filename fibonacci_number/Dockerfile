# --- Stage 1: Build ---
FROM rust:1.77 AS builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM rust:1.77-slim

WORKDIR /app

# Copy only the compiled binary
COPY --from=builder /app/target/release/fibonacci_number /usr/local/bin/fibonacci_number

# Set command to run the app
CMD ["fibonacci_number"]
