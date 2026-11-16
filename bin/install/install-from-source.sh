#!/usr/bin/env sh

set -eu

cd "$(dirname "$0")/../.."

EXECUTABLE_NAME="ticket-commit-msg"

cargo build --release

TARGET_DIR="${HOME}/.local/bin"
mkdir -p "${TARGET_DIR}"

# Renamed into place rather than copied over the old file: on Windows `cp` onto
# a running process's executable fails with "Device or resource busy".
cp "$(pwd)/target/release/${EXECUTABLE_NAME}" "${TARGET_DIR}/${EXECUTABLE_NAME}.tmp"
mv -f "${TARGET_DIR}/${EXECUTABLE_NAME}.tmp" "${TARGET_DIR}/${EXECUTABLE_NAME}"
chmod +x "${TARGET_DIR}/${EXECUTABLE_NAME}"

echo "Installed: ${TARGET_DIR}/${EXECUTABLE_NAME}"
