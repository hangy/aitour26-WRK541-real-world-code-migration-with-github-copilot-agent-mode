# Weather API - Rust Implementation

A high-performance weather API built with Actix-web and Rust, migrated from the Python/FastAPI version.

## Features

- Fast, type-safe weather data API
- Historical temperature data for multiple cities worldwide
- OpenAPI/Swagger documentation
- Comprehensive test coverage

## Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

The server will start on `http://localhost:8000`

## Docker

### Build Docker Image

```bash
make docker-build
# or
docker build -t weather-api:latest .
```

### Run with Docker Compose

```bash
make docker-run
# or
docker-compose up -d
```

The API will be available at `http://localhost:8000`

### Stop Docker Container

```bash
make docker-stop
# or
docker-compose down
```

### Docker Best Practices Used

- **Multi-stage build**: Separates build and runtime stages
- **cargo-chef**: Optimal dependency caching - only rebuilds changed dependencies
- **Distroless base image**: Minimal attack surface, only contains application and runtime dependencies
- **Binary stripping**: Reduces binary size by removing debug symbols
- **Small image size**: Distroless images are significantly smaller than full OS images (~50-70 MB)
- **.dockerignore**: Excludes unnecessary files from build context

The Dockerfile uses [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) to intelligently cache dependencies, making iterative builds much faster.

## Testing

Run all tests:
```bash
cargo test
```

Run shell script integration tests:
```bash
cd ..
bash tests/test_endpoints.sh
```

## API Endpoints

- `GET /` - Redirects to API documentation
- `GET /countries` - Returns list of available countries
- `GET /countries/{country}/{city}/{month}` - Returns temperature data

## Development

### Project Structure

```
src/
├── main.rs       # Application entry point
├── lib.rs        # Library exports
├── models.rs     # Data structures
├── data.rs       # Data loading logic
└── routes.rs     # API endpoints
```

### Running in Development Mode

```bash
cargo run
```

With auto-reload (requires cargo-watch):
```bash
cargo install cargo-watch
cargo watch -x run
```
