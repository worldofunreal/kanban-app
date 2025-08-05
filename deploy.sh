#!/bin/bash

echo "🚀 Deploying Kanban App..."

# Generate candid file
echo "📝 Generating candid interface..."
./generate-candid.sh

# Deploy all canisters
echo "🚀 Deploying canisters..."
dfx deploy backend && dfx generate
