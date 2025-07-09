#!/bin/bash

# Simple Rust Example Runner
# Usage: ./run_example.sh examples/activities/get_portfolio_activities.rs -- arg1 arg2

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <rust_file.rs> [-- <program_args...>]"
    echo "Example: $0 examples/activities/get_portfolio_activities.rs"
    echo "Example: $0 examples/activities/get_activity.rs -- 5e1a7df8-73de-49cb-8491-8d78e410b68c"
    exit 1
fi

RUST_FILE="$1"

if [ ! -f "$RUST_FILE" ]; then
    echo "Error: File '$RUST_FILE' not found"
    exit 1
fi

# Check if there are arguments after --
PROGRAM_ARGS=""
FOUND_DASH_DASH=false
for arg in "$@"; do
    if [ "$FOUND_DASH_DASH" = true ]; then
        if [ -z "$PROGRAM_ARGS" ]; then
            PROGRAM_ARGS="$arg"
        else
            PROGRAM_ARGS="$PROGRAM_ARGS $arg"
        fi
    elif [ "$arg" = "--" ]; then
        FOUND_DASH_DASH=true
    fi
done

if [ -n "$PROGRAM_ARGS" ]; then
    echo "Program arguments: $PROGRAM_ARGS"
fi

echo "Running: $RUST_FILE"

# Get file name without extension
FILE_NAME=$(basename "$RUST_FILE" .rs)

# Create temporary directory
TEMP_DIR=$(mktemp -d)
echo "Created temp directory: $TEMP_DIR"

# Create temporary Cargo.toml
cat > "$TEMP_DIR/Cargo.toml" << EOF
[package]
name = "temp_example"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "$FILE_NAME"
path = "$(pwd)/$RUST_FILE"

[dependencies]
prime_rs_sdk = { path = "$(pwd)" }
core_rs = { path = "$(pwd)/../core_rs" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
hmac = "0.12"
sha2 = "0.10"
base64 = "0.22"
async-trait = "0.1"
dotenv = "0.15"
log = "0.4"
env_logger = "0.11"
uuid = { version = "1.17", features = ["v4"] }
EOF

# Copy .env file if it exists
if [ -f ".env" ]; then
    cp .env "$TEMP_DIR/"
    echo "Copied .env file"
fi

# Change to temp directory and run
cd "$TEMP_DIR"
echo "Building..."

if cargo build --bin "$FILE_NAME"; then
    echo "Build successful! Running..."
    echo "----------------------------------------"
    if [ -n "$PROGRAM_ARGS" ]; then
        cargo run --bin "$FILE_NAME" -- $PROGRAM_ARGS
    else
        cargo run --bin "$FILE_NAME"
    fi
else
    echo "Build failed"
    exit 1
fi

# Clean up
cd - > /dev/null
rm -rf "$TEMP_DIR"
echo "Cleaned up temporary files" 