#!/bin/bash

# Test script for HTTP server
# This script starts the server, runs a curl test, and shuts down the server

set -e

# Color codes
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting HTTP server...${NC}"
cargo run &
SERVER_PID=$!

# Wait for server to start
echo -e "${YELLOW}Waiting for server to start...${NC}"
sleep 2

# Run curl test
echo -e "${GREEN}Running curl test...${NC}"
curl -v http://localhost:4221/test

echo ""
echo -e "${YELLOW}Test completed. Shutting down server...${NC}"

# Shutdown the server
kill $SERVER_PID

# Wait for the process to terminate
wait $SERVER_PID 2>/dev/null || true

echo -e "${RED}Server stopped.${NC}"
