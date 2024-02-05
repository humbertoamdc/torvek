#!/bin/bash

# Colors
Red='\033[0;31m'
Yellow='\033[0;33m'
NoColor='\033[0m'

ORY_PORT=4002

# Function to handle cleanup
function cleanup() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S') ${Red}EXIT${NoColor}] Cleaning up..."
    lsof -i :$ORY_PORT -t | xargs kill
    exit
}

# Trap CTRL+C and call cleanup function
trap cleanup INT

ory tunnel http://127.0.0.1:8082 \
    --port $ORY_PORT \
    --project priceless-easley-mvim77stn4 \
    --allowed-cors-origins http://127.0.0.1:8082 \
    --cookie-domain 127.0.0.1:8082 \
    --dev \
    &

env $(cat env/dev.env | xargs) trunk serve --port 8082;
