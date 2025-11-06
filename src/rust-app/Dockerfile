# Build stage with cargo-chef for better layer caching
FROM rust:1.83-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Prepare recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this layer is cached as long as dependencies don't change
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build application
COPY . .
RUN cargo build --release && \
    strip target/release/weather-api

# Runtime stage using distroless
FROM gcr.io/distroless/cc-debian12

# Copy the binary from builder
COPY --from=builder /app/target/release/weather-api /app/weather-api

# Copy the weather.json data file
COPY --from=builder /app/weather.json /app/weather.json

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8000

# Run the binary
CMD ["/app/weather-api"]
