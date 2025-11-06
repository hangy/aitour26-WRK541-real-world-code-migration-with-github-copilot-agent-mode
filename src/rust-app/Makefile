.PHONY: help build run test clean install dev docker-build docker-run docker-stop docker-clean

help:
	@echo "Available targets:"
	@echo "  build        - Build the project in release mode"
	@echo "  run          - Run the application"
	@echo "  test         - Run all tests"
	@echo "  dev          - Run in development mode with auto-reload"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install development dependencies"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run application in Docker container"
	@echo "  docker-stop  - Stop Docker container"
	@echo "  docker-clean - Remove Docker image and container"

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

docker-build:
	docker build -t weather-api:latest .

docker-run:
	docker-compose up -d

docker-stop:
	docker-compose down

docker-clean:
	docker-compose down --rmi all -v
