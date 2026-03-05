#!/bin/bash
# Build script for Tej Web

set -e

echo "Building Tej Web Speed Test..."

# Check for wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build WASM module
echo "Building WASM module..."
cd crates/tej-core
wasm-pack build --target web --out-dir ../../apps/tej-web/pkg

echo "Build complete!"
echo ""
echo "To test locally:"
echo "  cd apps/tej-web && python3 -m http.server 8000"
echo ""
echo "To deploy to Netlify:"
echo "  1. Push to GitHub"
echo "  2. Connect repo to Netlify"
echo "  3. Build command: cd crates/tej-core && wasm-pack build --target web --out-dir ../../apps/tej-web/pkg"
echo "  4. Publish directory: apps/tej-web"
