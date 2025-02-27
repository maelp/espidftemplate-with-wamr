#!/usr/bin/env bash
set -eou pipefail

# Create resources directory if it doesn't exist
mkdir -p resources/plugins

echo "Building Rust plugin..."
cd src/plugins/rust
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/rust_plugin.wasm ../../../resources/plugins/
cd ../../..

echo "Building AssemblyScript plugin..."
cd src/plugins/assemblyscript
npm install
npm run build
cd ../../..

echo "Building C plugin..."
# Check if emcc (Emscripten compiler) is available
if command -v emcc &> /dev/null; then
    cd src/plugins/c
    make
    cd ../../..
    echo "C plugin built successfully"
else
    echo "Warning: emcc (Emscripten compiler) not found, skipping C plugin build"
fi

echo "Plugins built successfully!"