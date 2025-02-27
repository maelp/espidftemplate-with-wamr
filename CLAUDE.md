# ESP-IDF Template with WAMR Development Guide

## Build Commands
- Build, flash and monitor: `./build_flash_monitor.sh` or `cargo run --release`
- Build only: `cargo build --release`
- Docker build: `docker run -v$(pwd):/workspace -it espressif/idf-rust:esp32_1.84.0.0`

## Environment Setup
- ESP setup: `cargo install espup && espup install`
- ESP-IDF: Clone v5.4 from github.com/espressif/esp-idf.git

## Code Style Guidelines
- Use 2018 edition Rust
- Follow Rust standard naming conventions (snake_case for functions/variables, CamelCase for types)
- Use the `log` crate for logging with appropriate levels (info, debug, warn, error)
- Handle errors with proper propagation using `?` operator when possible
- Add comments for non-obvious code sections
- Use rustfmt for consistent formatting
- Keep functions focused and short
- Organize imports in logical groups with standard library first