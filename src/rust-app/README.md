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
