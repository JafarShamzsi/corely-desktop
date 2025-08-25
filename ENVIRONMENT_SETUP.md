# Environment Configuration Guide

## Overview

This guide explains how to configure environment variables for the AI Training Marketplace Desktop Application, including authentication setup and local development configuration.

## 📁 Environment Files

### `.env.example` 
Template file with all available environment variables and documentation. **Safe to commit to repository.**

### `.env`
Local environment file with actual values for development/testing. **Never commit this file - it's in .gitignore.**

## 🚀 Quick Setup

### 1. Copy Environment Template

```bash
# Copy the example file to create your local environment
cp .env.example .env
```

### 2. Configure Basic Settings

Edit `.env` and set these essential variables:

```bash
# Generate a strong JWT secret (32+ characters)
JWT_SECRET=your-secure-random-jwt-secret-key-minimum-32-characters

# Set your environment
APP_ENV=development
LOG_LEVEL=info
DEBUG=true
```

### 3. Test the Application

You can now run the application with basic authentication:

```bash
# Start the development server
npm run tauri dev
```

The app will work with username/password authentication immediately. OAuth2 is optional.

## 🔐 OAuth2 Setup (Optional)

### Google OAuth2 Configuration

1. **Create Google Cloud Project**
   - Go to [Google Cloud Console](https://console.cloud.google.com/)
   - Create a new project or select existing
   - Enable the Google+ API

2. **Create OAuth2 Credentials**
   - Go to "Credentials" in the API Console
   - Click "Create Credentials" → "OAuth 2.0 Client IDs"
   - Choose "Web application"
   - Add authorized redirect URI: `http://localhost:1420/auth/callback`

3. **Configure Environment**
   ```bash
   GOOGLE_CLIENT_ID=your-google-client-id
   GOOGLE_CLIENT_SECRET=your-google-client-secret
   ```

### GitHub OAuth2 Configuration

1. **Create GitHub OAuth App**
   - Go to [GitHub Developer Settings](https://github.com/settings/developers)
   - Click "New OAuth App"
   - Set Application name: "AI Training Desktop (Dev)"
   - Set Authorization callback URL: `http://localhost:1420/auth/callback`

2. **Configure Environment**
   ```bash
   GITHUB_CLIENT_ID=your-github-client-id
   GITHUB_CLIENT_SECRET=your-github-client-secret
   ```

## 📋 Complete Environment Variables Reference

### Authentication & Security

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `JWT_SECRET` | **Yes** | - | Secret for JWT token signing (32+ chars) |
| `GOOGLE_CLIENT_ID` | No | - | Google OAuth2 client ID |
| `GOOGLE_CLIENT_SECRET` | No | - | Google OAuth2 client secret |
| `GITHUB_CLIENT_ID` | No | - | GitHub OAuth2 client ID |
| `GITHUB_CLIENT_SECRET` | No | - | GitHub OAuth2 client secret |
| `OAUTH_REDIRECT_URL` | No | `http://localhost:1420/auth/callback` | OAuth2 redirect URL |

### Application Configuration

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `APP_ENV` | No | `development` | Application environment |
| `LOG_LEVEL` | No | `info` | Logging level (trace, debug, info, warn, error) |
| `DEBUG` | No | `true` | Enable debug mode |
| `FRONTEND_PORT` | No | `1420` | Frontend development server port |

### Security Settings

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `ENABLE_RATE_LIMITING` | No | `false` | Enable API rate limiting |
| `ENABLE_AUDIT_LOGGING` | No | `true` | Enable security audit logging |
| `SESSION_TIMEOUT_MINUTES` | No | `60` | Session timeout in minutes |
| `REFRESH_TOKEN_DAYS` | No | `30` | Refresh token validity in days |

### Development Settings

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `HOT_RELOAD` | No | `true` | Enable hot reload in development |
| `DEV_TOOLS` | No | `true` | Enable development tools |
| `SKIP_OAUTH_WARNINGS` | No | `false` | Skip OAuth2 setup warnings |

## 🛠️ Development Environments

### Local Development (Default)

```bash
APP_ENV=development
LOG_LEVEL=info
DEBUG=true
HOT_RELOAD=true
DEV_TOOLS=true
SKIP_OAUTH_WARNINGS=true
```

### Testing Environment

```bash
APP_ENV=testing
LOG_LEVEL=debug
DEBUG=true
ENABLE_AUDIT_LOGGING=true
SESSION_TIMEOUT_MINUTES=30
```

### Production Environment

```bash
APP_ENV=production
LOG_LEVEL=warn
DEBUG=false
ENABLE_RATE_LIMITING=true
ENABLE_AUDIT_LOGGING=true
SESSION_TIMEOUT_MINUTES=120
# Use strong, unique JWT_SECRET
# Configure OAuth2 with production URLs
```

## 🔒 Security Best Practices

### JWT Secret Generation

Generate a cryptographically secure JWT secret:

```bash
# Using OpenSSL (recommended)
openssl rand -hex 32

# Using Node.js
node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"

# Using Python
python -c "import secrets; print(secrets.token_hex(32))"
```

### Environment File Security

1. **Never commit `.env` files** - they're in `.gitignore`
2. **Use different secrets** for each environment
3. **Rotate secrets regularly** in production
4. **Limit access** to environment files on servers
5. **Use secure storage** for production secrets (AWS Secrets Manager, Azure Key Vault, etc.)

## 🚨 Troubleshooting

### Common Issues

1. **JWT_SECRET not set**
   ```
   Error: Failed to initialize authentication manager
   ```
   **Solution**: Set a valid JWT_SECRET in your `.env` file

2. **OAuth2 redirect mismatch**
   ```
   OAuth2 error: redirect_uri_mismatch
   ```
   **Solution**: Ensure redirect URL in OAuth2 app matches `OAUTH_REDIRECT_URL`

3. **Invalid OAuth2 credentials**
   ```
   OAuth2 error: invalid_client
   ```
   **Solution**: Verify client ID and secret are correct

### Testing Authentication

Test the authentication system with these approaches:

1. **Username/Password** (Always available)
   - Register a new user
   - Login with credentials
   - Test token refresh

2. **Google OAuth2** (If configured)
   - Click "Login with Google"
   - Complete OAuth2 flow
   - Verify user profile data

3. **GitHub OAuth2** (If configured)
   - Click "Login with GitHub"
   - Complete OAuth2 flow
   - Verify user profile data

## 📚 Additional Resources

- [Google OAuth2 Setup Guide](https://developers.google.com/identity/protocols/oauth2)
- [GitHub OAuth2 Documentation](https://docs.github.com/en/developers/apps/building-oauth-apps)
- [JWT Best Practices](https://auth0.com/blog/a-look-at-the-latest-draft-for-jwt-bcp/)
- [Environment Variables Security](https://12factor.net/config)

## 🔄 Environment Updates

When updating environment variables:

1. **Update `.env.example`** with new variables and documentation
2. **Update this guide** with new variable descriptions
3. **Notify team members** to update their local `.env` files
4. **Update production** environment configurations
5. **Test thoroughly** after environment changes
