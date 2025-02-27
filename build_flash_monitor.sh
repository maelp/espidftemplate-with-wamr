#!/usr/bin/env bash
. ~/export-esp.sh
. ~/esp/esp-idf/export.sh
export CARGO_FEATURE_ESP_IDF=1 
export WAMR_BUILD_TARGET=XTENSA

cargo espflash flash --monitor --port=/dev/cu.usbmodem1101 --release --target=xtensa-esp32s3-espidf --chip esp32s3
