#!/bin/bash

# Colors
Red='\033[0;31m'
Yellow='\033[0;33m'
NoColor='\033[0m'

# Function to handle cleanup
function cleanup() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Red}EXIT${NoColor}] Cleaning up..."
    lsof -i :4001 -t | xargs kill
    exit
}

# Trap CTRL+C and call cleanup function
trap cleanup INT

# Start localstack
echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Yellow}INIT${NoColor}] Starting Ory tunnel"
#ory tunnel http://127.0.0.1:8081 \
#  --dev \
#  --port 4001 \
#  --project competent-merkle-esamqi0owk \
#  --allowed-cors-origins http://127.0.0.1:8081 \
#  &
#
# Start app
env $(cat env/dev.env | xargs) trunk serve --port 8081;

# If the server fails to start or CTRL+C is pressed, cleanup
cleanup

ory tunnel http://127.0.0.1:8081 --dev --port 4001 --project competent-merkle-esamqi0owk --allowed-cors-origins http://127.0.0.1:8081