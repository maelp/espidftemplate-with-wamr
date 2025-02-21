FROM espressif/idf-rust:esp32_1.84.0.0

USER root

# Install basic build tools
# Not sure if they are all needed...
RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essential \
  git \
  wget curl \
  flex \
  bison \
  gperf \
  python3 \
  python3-pip \
  python3-venv \
  cmake \
  ninja-build \
  ccache \
  libffi-dev \
  libssl-dev

# TODO: install stuff for WASM compilation of a test AssemblyScript/GO/Rust program

USER esp

WORKDIR /home/esp
