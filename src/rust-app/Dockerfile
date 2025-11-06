# Build stage
FROM rust:1.83-bookworm AS builder

WORKDIR /app

# Copy the entire project
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application
# Note: Dependencies will be cached as long as Cargo.toml and Cargo.lock don't change
RUN cargo build --release

# Strip the binary to reduce size
RUN strip target/release/weather-api

# Runtime stage using distroless
FROM gcr.io/distroless/cc-debian12

# Copy the binary from builder
COPY --from=builder /app/target/release/weather-api /app/weather-api

# Copy the weather.json data file
COPY weather.json /app/weather.json

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8000

# Run the binary
CMD ["/app/weather-api"]
