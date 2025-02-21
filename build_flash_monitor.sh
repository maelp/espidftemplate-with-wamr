#!/usr/bin/env bash
cargo espflash flash --monitor --port=/dev/cu.usbmodem1101 --release --target=xtensa-esp32s3-espidf --chip esp32s3
