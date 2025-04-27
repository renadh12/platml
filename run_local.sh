#!/bin/bash

# Create the model storage directory
mkdir -p ./model_storage/models

# Start the model management API
echo "Starting Model Management API on port 8081..."
cd api
cargo build
(cargo run &)
API_PID=$!

# Wait a moment to ensure API is starting
sleep 2

# Start the model serving API
echo "Starting Model Serving API on port 8080..."
cd ../serve
cargo build
(cargo run &)
SERVE_PID=$!

echo "APIs are running. Press Ctrl+C to stop."
trap "kill $API_PID $SERVE_PID; exit" INT TERM EXIT

# Keep the script running
wait