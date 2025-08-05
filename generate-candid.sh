#!/bin/bash

# Generate .did file from Rust code
echo "Generating .did file from Rust code..."
cargo build --target wasm32-unknown-unknown --release -p backend
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > src/backend/backend.did

echo "Generated .did file at src/backend/backend.did" 