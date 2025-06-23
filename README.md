# Portfolio Backend

A portfolio management backend built with Rust, featuring secure authentication, portfolio tracking, asset management, and integration with external APIs like CoinGecko Portfolio.

> **Note**: This project is created for learning purposes to explore Rust web development, modern API design, and portfolio management systems.

## Features

- **User Authentication**: Secure JWT-based authentication with Argon2 password hashing
- **Portfolio Management**: Create and manage multiple investment portfolios
- **Asset Tracking**: Support for crypto and stock assets with real-time data
- **Transaction History**: Comprehensive transaction tracking with detailed metadata
- **External API Integration**: CoinGecko integration for cryptocurrency data
- **File Import**: Import portfolio data from CoinGecko export files
- **RESTful API**: Well-documented REST API with OpenAPI/Swagger documentation
- **Database Migrations**: SQLx-based database migrations for PostgreSQL
- **Docker Support**: Complete containerization with Docker Compose
- **Health Monitoring**: Built-in health checks and monitoring endpoints

## Tech Stack

- **Language**: Rust 2024 Edition
- **Web Framework**: Axum
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Authentication**: JWT with Argon2 password hashing
- **API Documentation**: OpenAPI/Swagger with utoipa
- **HTTP Client**: Reqwest
- **Serialization**: Serde
- **Logging**: Tracing
- **Configuration**: Config crate with TOML and environment variables
- **Containerization**: Docker & Docker Compose

## Prerequisites

- Rust (latest stable version)
- Docker & Docker Compose
- PostgreSQL 17.4+
- Redis 7.4+

## Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd portfolio_be
```

### 2. Set Up Configuration

Copy the template configuration file:

```bash
cp settings.template.toml settings.toml
```

Edit `settings.toml` with your configuration:

```toml
[server]
port = 4000
secret_key = "your-secret-key-here"
allow_origins = ["http://localhost:3000"]

[postgres]
url = "postgres://portfolio:Portfolio@localhost:5432/portfolio_local"

[redis]
url = "redis://localhost:6379"

[clients.coingecko]
api_key = "your-coingecko-api-key"

[logging]
level = "debug"
```

### 3. Start Dependencies with Docker

```bash
docker-compose -f docker/docker-compose.yml up -d
```

This will start:

- PostgreSQL on port 5432
- Redis on port 6379

### 4. Run Database Migrations

```bash
cargo install sqlx-cli
sqlx migrate run
```

### 5. Build and Run the Application

```bash
cargo build
cargo run
```

The server will start on `http://localhost:4000`

## API Documentation

Once the server is running, you can access the interactive API documentation at:

- **Swagger UI**: `http://localhost:4000/docs`
- **OpenAPI JSON**: `http://localhost:4000/api-docs/openapi.json`

## Authentication

The API uses JWT-based authentication. To access protected endpoints:

1. **Sign Up**: `POST /auth/signup`
2. **Login**: `POST /auth/login_with_password`
3. Include the JWT token in the `Authorization` header: `Bearer <token>`

### Example Authentication Flow

```bash
# Sign up
curl -X POST http://localhost:4000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123", "name": "John Doe"}'

# Login
curl -X POST http://localhost:4000/auth/login_with_password \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

## Core Endpoints

### Portfolios

- `GET /api/portfolios` - Get user's portfolios
- `POST /api/portfolios` - Create a new portfolio
- `GET /api/portfolios/{id}` - Get portfolio details
- `POST /api/portfolios/{id}/assets` - Add asset to portfolio

### Assets

- `GET /api/assets` - Get all available assets

### Imports

- `POST /api/imports/upload_portfolio_file` - Import CoinGecko portfolio file
- `GET /api/imports/coin_data/{id}` - Get coin data from CoinGecko

### Users

- `GET /api/users/me` - Get current user profile
- `GET /api/users/{id}` - Get user by ID

## Database Schema

### Core Tables

- **users**: User accounts and authentication
- **user_sessions**: Active user sessions
- **portfolios**: User investment portfolios
- **assets**: Available assets (crypto, stocks)
- **portfolio_assets**: Many-to-many relationship between portfolios and assets
- **transactions**: Portfolio transaction history

### Key Relationships

- Users can have multiple portfolios
- Portfolios can contain multiple assets
- Each transaction is linked to a portfolio and asset
- Assets can be shared across multiple portfolios

## Development

### Database Migrations

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert migrations
sqlx migrate revert
```

### Docker Development

For development with Docker:

```bash
# Build and run with Docker Compose
docker-compose -f docker/dev/docker-compose.yml up --build
```

## 🔒 Security Features

- **Password Hashing**: Argon2 for secure password storage
- **JWT Authentication**: Stateless authentication with configurable expiration
- **CORS Protection**: Configurable CORS policies
- **Request Tracing**: Request ID tracking for debugging
- **Input Validation**: Comprehensive request validation
- **SQL Injection Protection**: Parameterized queries with SQLx

## 📈 Monitoring & Health Checks

- **Health Endpoint**: `GET /health` - Application health status
- **Database Health**: Automatic database connectivity checks
- **Structured Logging**: JSON-formatted logs with tracing
- **Request Tracing**: Unique request IDs for debugging

## 🚀 Deployment

### Environment Variables

The application supports configuration via environment variables with the `APP_` prefix:

```bash
export APP_SERVER_PORT=4000
export APP_SERVER_SECRET_KEY="your-secret-key"
export APP_POSTGRES_URL="postgres://user:pass@host:port/db"
export APP_REDIS_URL="redis://host:port"
export APP_CLIENTS_COINGECKO_API_KEY="your-api-key"
```

### Production Considerations

1. **Database**: Use a managed PostgreSQL service
2. **Redis**: Use a managed Redis service
3. **Secrets**: Store sensitive configuration in environment variables
4. **Logging**: Configure structured logging for production
5. **CORS**: Restrict allowed origins to your frontend domain
6. **Rate Limiting**: Consider implementing rate limiting for production

Built with ❤️ using Rust and Axum
