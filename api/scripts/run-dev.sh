#!/bin/bash

# Colors
Red='\033[0;31m'
Yellow='\033[0;33m'
NoColor='\033[0m'

# Function to handle cleanup
function cleanup() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Red}EXIT${NoColor}] Cleaning up..."
    pgrep stripe | xargs kill
    docker compose down
    exit
}

# Trap CTRL+C and call cleanup function
trap cleanup INT

# Forward stripe webhooks to local environment
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Yellow}INIT${NoColor}] Forwarding stripe webhooks to local environment"
stripe listen --forward-to 127.0.0.1:3000/api/v1/payments/webhooks/complete_checkout_session &

# Start localstack
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Yellow}INIT${NoColor}] Starting local AWS infra"
docker compose up -d --wait

# Kill anything running on port 3000
lsof -i :3000 -t | xargs kill

# Start server
AWS_ACCESS_KEY_ID=test \
AWS_SECRET_ACCESS_KEY=test \
AWS_ENDPOINT_URL=http://127.0.0.1:4566 \
RUST_LOG=debug \
cargo-watch -x run

# If the server fails to start or CTRL+C is pressed, cleanup
cleanup
