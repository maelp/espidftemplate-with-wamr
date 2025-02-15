FROM espressif/idf-rust:esp32_1.84.0.0

USER root

RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essential \
  git \
  wget \
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
  vim

USER esp

WORKDIR /home/esp

# install espressif idf.py etc, since this is needed for the C++ build, I figured it might be useful to build the wrapper?
RUN mkdir -p ~/esp && cd ~/esp && git clone -b v5.4 --recursive https://github.com/espressif/esp-idf.git
RUN cd ~/esp/esp-idf && ./install.sh
