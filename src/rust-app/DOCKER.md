# Docker Configuration for Weather API

## Overview

The Weather API uses an optimized Dockerfile with cargo-chef for production-grade Rust containerization.

## Dockerfile Architecture

The Dockerfile uses **cargo-chef** for intelligent dependency caching:

**How it works:**
1. **Planner stage**: Analyzes your project and creates a "recipe" of dependencies
2. **Builder stage**: Builds only the dependencies that changed
3. **Final build**: Compiles your actual application code
4. **Runtime stage**: Minimal distroless image with just the binary

**Benefits:**
- Only rebuilds dependencies when they actually change
- Source code changes don't trigger full dependency rebuild
- Significantly faster iterative builds (seconds vs minutes)
- Industry standard for Rust Docker builds

**Usage:**
```bash
docker build -t weather-api:latest .
```

## Best Practices Implemented

### Multi-Stage Build
- **Builder stage**: Uses full Rust toolchain (rust:1.83-bookworm)
- **Runtime stage**: Uses minimal distroless image (gcr.io/distroless/cc-debian12)
- Result: ~90% smaller final image

### Distroless Base Image
- No shell, package manager, or unnecessary binaries
- Reduces attack surface significantly
- Only includes required runtime dependencies
- Perfect for production deployments

### Binary Optimization
- Release build with optimizations (`--release`)
- Debug symbols stripped to reduce size
- Typically results in 5-10 MB binaries

### Layer Caching Strategy
- Dependencies cached separately from source code
- Cargo.toml and Cargo.lock changes trigger dependency rebuild
- Source code changes don't affect dependency layer

## Image Size Comparison

Typical sizes:
- **With distroless**: ~50-70 MB
- **With debian-slim**: ~150-200 MB  
- **With full debian**: ~300-400 MB

## Security Benefits

1. **Minimal attack surface** - No unnecessary tools or libraries
2. **No shell access** - Prevents shell-based attacks
3. **Smaller CVE footprint** - Fewer packages = fewer vulnerabilities
4. **Immutable infrastructure** - Distroless images are designed to be immutable

## Docker Compose

The included `docker-compose.yml` provides:
- Easy single-command deployment
- Port mapping (8000:8000)
- Automatic restart policy
- Health checks
- Environment variable management

## Running the Container

### With Docker Compose (Recommended)
```bash
docker-compose up -d
```

### With Docker CLI
```bash
docker build -t weather-api:latest .
docker run -d -p 8000:8000 --name weather-api weather-api:latest
```

## Verifying the Deployment

```bash
# Check container status
docker ps

# Check logs
docker logs weather-api

# Test the API
curl http://localhost:8000/countries

# Access Swagger UI
open http://localhost:8000/docs/
```

## Stopping and Cleaning Up

```bash
# Stop container
docker-compose down

# Remove everything including images
docker-compose down --rmi all -v
```

## Production Considerations

1. **Environment Variables**: Configure via docker-compose.yml or -e flags
2. **Volume Mounts**: Consider mounting weather.json as volume for easy updates
3. **Health Checks**: Included in docker-compose.yml for orchestration
4. **Resource Limits**: Add memory/CPU limits in production
5. **Logging**: Configure log drivers for centralized logging

## Troubleshooting

### Container won't start
```bash
docker logs weather-api
```

### Permission issues
Distroless runs as non-root by default. Ensure files are readable.

### Port conflicts
Change port mapping in docker-compose.yml: `"8080:8000"`

## Alternative: Development Container

For development with hot-reload:
```bash
docker run -it --rm \
  -v $(pwd):/app \
  -w /app \
  -p 8000:8000 \
  rust:1.83-bookworm \
  cargo watch -x run
```
