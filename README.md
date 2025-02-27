# ESP-Rust with WAMR for ESP32S3

This is a template project for ESP32S3 development with Rust, incorporating the WebAssembly Micro Runtime (WAMR) to load and run WebAssembly plugins.

The important settings are in `.cargo/config.toml`:

- The `CARGO_FEATURE_ESP_IDF` and `WAMR_BUILD_TARGET` env vars are set in the `[env]` section
- The flags for ESP32S3 are set in `rustflags`

## Build on your host machine

### Prerequisites (espup and esp-idf)

Install espup (this creates a `~/export-esp.sh`)

```sh
cargo install espup
espup install
```

Install esp-idf (this creates a `~/esp/esp-idf/export.sh`)

```sh
mkdir ~/esp
git clone -b v5.4 --recursive https://github.com/espressif/esp-idf.git
cd esp-idf && ./install.sh esp32s3
```

### Build WASM Plugins

Before building the main application, you should build the WASM plugins:

```sh
# Make the script executable if needed
chmod +x build_plugins.sh

# Build the plugins
./build_plugins.sh
```

This will build:
1. A Rust plugin (compiled to WebAssembly)
2. An AssemblyScript plugin (compiled to WebAssembly)

Both plugins will be placed in the `resources/plugins` directory and embedded into the main application binary.

### Build, flash and monitor from the host

```sh
./build_flash_monitor.sh

# or equivalently

. ~/export-esp.sh
. ~/esp/esp-idf/export.sh
cargo run --release # does a build, flash and monitor, see `.cargo/config.toml` for details
```

## Build from Docker container

```sh
docker run -v$(pwd):/workspace -it espressif/idf-rust:esp32_1.84.0.0
# in the docker shell
cd /workspace
./build_plugins.sh
cargo build --release
```

## Creating Custom WASM Plugins

You can create custom plugins in various languages that compile to WebAssembly:

1. **Rust Plugins**: Use `cargo build --target wasm32-unknown-unknown`
2. **AssemblyScript Plugins**: Use `npm run build` with appropriate `.ts` source files
3. **C/C++ Plugins**: Use Emscripten or other WASM compilers

Plugins need to:
1. Export functions (like `print_message` and `add` in our examples)
2. Import host functions when needed (like `log_message`)

The ESP32 host provides these imported functions, allowing for bidirectional communication.
