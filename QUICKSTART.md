# Quick Start Guide

## Option 1: Docker Compose (Recommended - Easiest)

### Prerequisites
- Docker Desktop installed and running

### Steps

1. **Start all services:**
   ```bash
   docker-compose up -d
   ```

2. **Check service status:**
   ```bash
   docker-compose ps
   ```

3. **View application logs:**
   ```bash
   docker-compose logs -f telemetrywatch
   ```

4. **Access services:**
   - **TelemetryWatch API**: http://localhost:8080
   - **Prometheus**: http://localhost:9090
   - **Grafana**: http://localhost:3000 (username: `admin`, password: `admin12345`)
   - **PostgreSQL**: localhost:5432

5. **Test endpoints:**
   ```bash
   # Health check
   curl http://localhost:8080/health
   
   # Readiness check
   curl http://localhost:8080/ready
   
   # Metrics
   curl http://localhost:8080/metrics
   
   # Status
   curl http://localhost:8080/api/v1/status
   ```

6. **Stop services:**
   ```bash
   docker-compose down
   ```

## Option 2: Local Development (Requires Rust)

### Prerequisites
- Rust 1.75+ installed
- PostgreSQL running locally

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Steps

1. **Start PostgreSQL:**
   ```bash
   docker run -d \
     --name postgresql \
     -e POSTGRES_USER=telemetrywatch \
     -e POSTGRES_PASSWORD=telemetrywatch \
     -e POSTGRES_DB=telemetrywatch \
     -p 5432:5432 \
     postgres:15-alpine
   ```

2. **Create .env file:**
   ```bash
   cat > .env << EOF
   HOST=0.0.0.0
   PORT=8080
   DATABASE_URL=postgresql://telemetrywatch:telemetrywatch@localhost:5432/telemetrywatch
   DATABASE_MAX_CONNECTIONS=10
   METRICS_ENABLED=true
   EOF
   ```

3. **Build and run:**
   ```bash
   cargo build --release
   cargo run --release
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

## Option 3: Kubernetes Deployment

### Prerequisites
- Kubernetes cluster (1.20+)
- kubectl configured

### Steps

1. **Apply all manifests:**
   ```bash
   kubectl apply -f k8s/
   ```

2. **Check deployment:**
   ```bash
   kubectl get pods -n telemetrywatch
   kubectl get svc -n telemetrywatch
   ```

3. **Access Grafana (NodePort):**
   ```bash
   # Get NodePort
   kubectl get svc grafana-service -n telemetrywatch
   
   # Access via <node-ip>:<nodeport>
   ```

## Troubleshooting

### Docker Compose Issues

**Services not starting:**
```bash
# Check logs
docker-compose logs

# Restart services
docker-compose restart

# Rebuild images
docker-compose up -d --build
```

**Port already in use:**
- Change ports in `docker-compose.yml` if 8080, 9090, 3000, or 5432 are taken

### Local Development Issues

**Database connection errors:**
- Ensure PostgreSQL is running
- Check DATABASE_URL in .env file
- Verify PostgreSQL is accessible on port 5432

**Build errors:**
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

