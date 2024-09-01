#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
"" | "release")
    BUILD_MODE="release"
    ;;
"debug")
    BUILD_MODE="debug"
    ;;
*)
    echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
    exit 1
    ;;
esac

espflash flash --chip esp32s3 -p /dev/ttyUSB1 --list-all-ports target/xtensa-esp32s3-espidf/${BUILD_MODE}/weatherstation 