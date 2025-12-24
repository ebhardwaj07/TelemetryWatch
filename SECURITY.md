# Security Guidelines

## Important Security Notes

**⚠️ Never commit passwords or secrets to the repository!**

This repository contains example/default passwords for development purposes only. For production deployments, you **MUST** change all default passwords and use proper secret management.

## Default Passwords (Development Only)

The following default credentials are provided for local development:

- **Grafana**: `admin` / `admin12345` (or set via `GF_SECURITY_ADMIN_PASSWORD`)
- **PostgreSQL**: `telemetrywatch` / `telemetrywatch` (or set via `POSTGRES_PASSWORD`)

**These should NEVER be used in production!**

## Securing Your Deployment

### For Docker Compose

1. Create a `.env` file (copy from `env.example`):
   ```bash
   cp env.example .env
   ```

2. Edit `.env` and set strong, unique passwords:
   ```bash
   GF_SECURITY_ADMIN_PASSWORD=your_strong_grafana_password
   POSTGRES_PASSWORD=your_strong_postgres_password
   DATABASE_URL=postgresql://telemetrywatch:your_strong_postgres_password@postgresql:5432/telemetrywatch
   ```

3. The `.env` file is already in `.gitignore` and will not be committed.

### For Kubernetes

1. Create Kubernetes secrets:
   ```bash
   kubectl create secret generic telemetrywatch-secrets \
     --from-literal=grafana-admin-password='your_strong_grafana_password' \
     --from-literal=postgres-password='your_strong_postgres_password' \
     -n telemetrywatch
   ```

2. Update the Kubernetes manifests to reference these secrets instead of hardcoded values.

3. Use a secrets management solution like:
   - HashiCorp Vault
   - AWS Secrets Manager
   - Azure Key Vault
   - Google Secret Manager

## Best Practices

1. **Use strong passwords**: Minimum 16 characters, mix of letters, numbers, and symbols
2. **Rotate passwords regularly**: Especially in production environments
3. **Use secrets management**: Never hardcode secrets in configuration files
4. **Limit access**: Use least-privilege principles
5. **Monitor access**: Enable audit logging for sensitive operations
6. **Use TLS/SSL**: Encrypt connections in production
7. **Keep dependencies updated**: Regularly update all components for security patches

## Reporting Security Issues

If you discover a security vulnerability, please report it responsibly:
- Do NOT create a public GitHub issue
- Contact the maintainers privately
- Provide detailed information about the vulnerability

