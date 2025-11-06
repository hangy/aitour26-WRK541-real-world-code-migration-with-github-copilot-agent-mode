.PHONY: help build run test clean install dev

help:
	@echo "Available targets:"
	@echo "  build   - Build the project in release mode"
	@echo "  run     - Run the application"
	@echo "  test    - Run all tests"
	@echo "  dev     - Run in development mode with auto-reload"
	@echo "  clean   - Clean build artifacts"
	@echo "  install - Install development dependencies"

build:
	cargo build --release

run:
	cargo run

test:
	cargo test

dev:
	cargo watch -x run

clean:
	cargo clean

install:
	cargo install cargo-watch
