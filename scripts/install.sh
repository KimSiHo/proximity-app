#!/usr/bin/env bash

set -e

PROFILE="${1:-release}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
ROOT_DIR="$(cd "${PROJECT_DIR}/.." && pwd)"

APP_NAME="app"

SRC="${PROJECT_DIR}/target/aarch64-unknown-linux-gnu/${PROFILE}/${APP_NAME}"
DST="${ROOT_DIR}/jetson-bsp/Linux_for_Tegra/rootfs/usr/local/bin/${APP_NAME}"

if [ ! -f "${SRC}" ]; then
    echo "Error: ${SRC} not found"
    exit 1
fi

sudo install -Dm755 "${SRC}" "${DST}"

echo "Installed:"
echo "  ${SRC}"
echo "    ->"
echo "  ${DST}"
