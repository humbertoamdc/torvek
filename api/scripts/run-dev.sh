#!/bin/bash

# Colors
Red='\033[0;31m'
Yellow='\033[0;33m'
NoColor='\033[0m'

# Function to handle cleanup
function cleanup() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Red}EXIT${NoColor}] Cleaning up..."
    docker compose down
    exit
}

# Trap CTRL+C and call cleanup function
trap cleanup INT

# Start localstack
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Yellow}INIT${NoColor}] Starting local AWS infra"
docker compose up -d --wait

# Start server
AWS_ACCESS_KEY_ID=test \
AWS_SECRET_ACCESS_KEY=test \
AWS_ENDPOINT_URL=http://127.0.0.1:4566 \
RUST_LOG=debug \
cargo-watch -x run

# If the server fails to start or CTRL+C is pressed, cleanup
cleanup
