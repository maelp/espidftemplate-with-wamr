# Simple template for esp-rust with wamr for esp32s3

## Prerequisites

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

## Build, flash and monitor from the host

The important settings are in `.cargo/config.toml`:

- the `CARGO_FEATURE_ESP_IDF` and `WAMR_BUILD_TARGET` env vars are set in the `[env]` section
- the flags for esp32s3 are set in `rustflags`

```sh
. ~/export-esp.sh
. ~/esp/esp-idf/export.sh
cargo run --release # does a build, flash and monitor, see `.cargo/config.toml` for details
```

## Build from Docker container

```sh
docker run  -v`pwd`:/workspace -it espressif/idf-rust:esp32_1.84.0.0
# in the docker shell
cd /workspace
cargo build --release
```
