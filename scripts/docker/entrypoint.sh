#!/bin/bash
set -e

# Wait for database to be ready (if using a database)
./wait-for-db.sh

# Run migrations (if using SQLx)
sqlx migrate run

# Start the application
exec ./your_app_name
