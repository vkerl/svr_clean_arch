# Clean Architecture Server

A Rust server implementation following clean architecture principles.

## Environment Variables

The server uses the following environment variables that should be defined in your `.env` file:

### Database Configuration
```env
# Database name for the application
DATABASE_NAME=test

# Full database connection URL
DATABASE_URL=postgres://<username>:<password>@<host>:<port>
```

### Server Configuration
```env
# Host address for the server to listen on
SERVER_HOST=127.0.0.1

# Port number for the server
SERVER_PORT=8080
```

### Logging Configuration
```env
# Log level (trace, debug, info, warn, error)
RUST_LOG=info
```

## Default Values

If environment variables are not set, the server will use these defaults:
- SERVER_HOST: "127.0.0.1"
- SERVER_PORT: "8080"
- DATABASE_NAME: "postgres"
- RUST_LOG: "info"