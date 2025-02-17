FROM debian:bookworm

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive \
  USERNAME=esp \
  HOME=/home/esp

# Install required dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essential \
  git \
  wget \
  curl \
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
  libssl-dev \
  dfu-util \
  libusb-1.0-0 \
  gcc-multilib \
  clang \
  vim \
  && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -s /bin/bash $USERNAME && \
  echo "$USERNAME ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

USER $USERNAME
WORKDIR $HOME

# Install Rust and required cargo tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
  echo "export PATH=\"$HOME/.cargo/bin:\$PATH\"" >> $HOME/.bashrc && \
  . "$HOME/.cargo/env" && \
  cargo install cargo-generate ldproxy espup espflash cargo-espflash

# Install Espressif toolchain with espup
RUN . "$HOME/.cargo/env" && \
  espup install && \
  echo "source $HOME/export-esp.sh" >> $HOME/.bashrc

# Clone and install ESP-IDF (for C++ build tools)
RUN mkdir -p $HOME/esp && cd $HOME/esp && git clone -b v5.4 --recursive https://github.com/espressif/esp-idf.git && \
  cd $HOME/esp/esp-idf && ./install.sh

# Set default shell to bash
CMD ["/bin/bash"]
