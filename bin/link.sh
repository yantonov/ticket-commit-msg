#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

TARGET_EXECUTABLE="${HOME}/bin/ticket-commit-msg"
if [ -L "${TARGET_EXECUTABLE}" ]; then
    rm "${TARGET_EXECUTABLE}"
fi

ln -s $(pwd)/target/release/ticket-commit-msg ${TARGET_EXECUTABLE}

TARGET_INSTALL_SCRIPT="${HOME}/bin/ticket-commit-msg-install"
if [ -L "${TARGET_INSTALL_SCRIPT}" ]; then
    rm "${TARGET_INSTALL_SCRIPT}"
fi

ln -s $(pwd)/install ${TARGET_INSTALL_SCRIPT}
