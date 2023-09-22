#!/bin/bash

if [ $# -lt 1 ]; then
    printf "Usage: rpiz_debug.sh <ssh_destination>\n\n\tssh_destination\n\t\tThe destination for connecting with SSH in format user@hostname e.g. pi@raspberrypi.local\n";
    exit;
fi

cargo build --target=arm-unknown-linux-gnueabihf

BUILD_TARGET="arm-unknown-linux-gnueabihf"
BUILD_TYPE="debug"

PKG_NAME=$(gawk 'BEGIN { in_package = false; } /^\[package\]/ { in_package = true; } /^name/ { if(in_package == true) { print $3; } } /^\[.*?\]/ { in_package = false; }' "Cargo.toml" | tr -d '"')
EXE_FILE="target/${BUILD_TARGET}/${BUILD_TYPE}/${PKG_NAME}"

scp "$EXE_FILE" "$1:~/${PKG_NAME}"
# TODO: Remote debugging with lldb