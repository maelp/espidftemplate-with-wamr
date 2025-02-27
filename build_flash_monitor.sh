#!/usr/bin/env bash
set -eou pipefail

if [ ! -e "~/export-esp.sh" ]; then
    echo "~/export-esp.sh does not exist"
    exit 1
fi
if [ ! -e "~/esp/esp-idf/export.sh" ]; then
    echo "~/esp/esp-idf/export.sh does not exist"
    exit 1
fi

. ~/export-esp.sh
. ~/esp/esp-idf/export.sh

cargo run --release