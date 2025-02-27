#!/usr/bin/env bash
set -eou pipefail

# Create resources directory if it doesn't exist
mkdir -p resources/plugins

echo "Building Rust plugin..."
cd plugins/rust
# Build with aggressive optimization for small size
RUSTFLAGS="-C opt-level=z -C lto=true -C codegen-units=1 -C panic=abort" cargo build --target wasm32-unknown-unknown --release
# # Optimize WASM size further with wasm-opt if available
# if command -v wasm-opt &> /dev/null; then
#     echo "Optimizing Rust WASM with wasm-opt..."
#     wasm-opt -Oz target/wasm32-unknown-unknown/release/rust_plugin.wasm -o target/wasm32-unknown-unknown/release/rust_plugin.opt.wasm
#     cp target/wasm32-unknown-unknown/release/rust_plugin.opt.wasm ../../resources/plugins/rust_plugin.wasm
# else
    cp target/wasm32-unknown-unknown/release/rust_plugin.wasm ../../resources/plugins/
# fi
cd ../..

echo "Building AssemblyScript plugin..."
cd plugins/assemblyscript
npm install
npm run build
cd ../..

echo "Building C plugin..."
# Check if emcc (Emscripten compiler) is available
# if command -v emcc &> /dev/null; then
    cd plugins/c
    make
    cd ../..
    echo "C plugin built successfully"
# else
#     echo "Warning: emcc (Emscripten compiler) not found, skipping C plugin build"
# fi

echo "Plugins built successfully!"