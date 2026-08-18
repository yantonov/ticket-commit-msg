#!/usr/bin/env sh

set -eu

cd "$(dirname "$0")/../.."

EXECUTABLE_NAME="ticket-commit-msg"

cargo build --release

TARGET_DIR="${HOME}/.local/bin"
mkdir -p "${TARGET_DIR}"

cp "$(pwd)/target/release/${EXECUTABLE_NAME}" "${TARGET_DIR}/${EXECUTABLE_NAME}"
chmod +x "${TARGET_DIR}/${EXECUTABLE_NAME}"

echo "Installed: ${TARGET_DIR}/${EXECUTABLE_NAME}"
