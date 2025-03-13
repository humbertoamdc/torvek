#!/bin/bash

# Colors
Red='\033[0;31m'
Yellow='\033[0;33m'
Green='\033[0;32m'
NoColor='\033[0m'

# Configuration
API_PORT=3001
MAX_RETRIES=30
RETRY_INTERVAL=2

# Function to check if API is ready
check_api_health() {
    curl -s "http://localhost:${API_PORT}/health" > /dev/null
    return $?
}

# Function to handle cleanup
function cleanup() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Red}EXIT${NoColor}] Cleaning up..."
    if [ ! -z "$API_PID" ]; then
        kill $API_PID 2>/dev/null || true
    fi
    docker compose -f docker-compose.test.yaml down -v
    exit
}

# Trap CTRL+C and call cleanup function
trap cleanup INT

# Build file converter image
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Yellow}INIT${NoColor}] Updating docker images"
docker compose -f docker-compose.test.yaml build

# Start test infrastructure
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Yellow}INIT${NoColor}] Starting test infrastructure"
docker compose -f docker-compose.test.yaml down -v || true
docker compose -f docker-compose.test.yaml up -d --wait

# Kill anything running on test port
lsof -i :${API_PORT} -t | xargs kill 2>/dev/null || true

API_PID=$!


# Run tests
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Green}START${NoColor}] Running tests..."
AWS_ACCESS_KEY_ID=test \
AWS_SECRET_ACCESS_KEY=test \
AWS_ENDPOINT_URL=http://127.0.0.1:4576 \
AWS_S3_ENDPOINT_URL=http://s3.localhost.localstack.cloud:4576 \
RUST_BACKTRACE=1 \
cargo test

TEST_EXIT_CODE=$?

# Cleanup
cleanup

# Exit with test exit code
exit $TEST_EXIT_CODE
