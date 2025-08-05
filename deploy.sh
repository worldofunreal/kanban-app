#!/bin/bash

echo "🚀 Deploying Kanban App..."

# Generate candid file
echo "📝 Generating candid interface..."
./generate-candid.sh

# Deploy all canisters
echo "🚀 Deploying canisters..."
dfx deploy

echo "✅ Deployment complete!"
echo ""
echo "🌐 Frontend: http://vizcg-th777-77774-qaaea-cai.localhost:4943/"
echo "🔧 Backend: http://127.0.0.1:4943/?canisterId=vpyes-67777-77774-qaaeq-cai&id=ufxgi-4p777-77774-qaadq-cai" 