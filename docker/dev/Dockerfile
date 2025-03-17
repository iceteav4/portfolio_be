# Use the official Rust image as base
FROM rust:1.85-slim-bookworm

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Create and set working directory
WORKDIR /app

# Install cargo-watch for development
RUN cargo install cargo-watch

# Copy the entire project
COPY . .

# Command to run the development server with hot reload
CMD ["cargo", "watch", "-x", "run"]
