# Build stage
FROM rust:1.83-bookworm AS builder

# Create a new empty shell project
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn create_app() -> actix_web::App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse, Error = actix_web::Error, InitError = ()>> { actix_web::App::new() }" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

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
