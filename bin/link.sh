#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

TARGET="${HOME}/bin/ticket-commit-msg"
if [ -L "${TARGET}" ]; then
    echo 'Remove old symlink for the hook'
    rm "${TARGET}"
fi

echo 'Create symlink for the hook'
ln -s $(pwd)/target/release/ticket-commit-msg ${TARGET}

TARGET="${HOME}/bin/ticket-commit-msg-install"
if [ -L "${TARGET}" ]; then
    echo 'Remove old symlink for install script'
    rm "${TARGET}"
fi

echo 'Create symlink for install script'
ln -s $(pwd)/install ${TARGET}

echo 'Done'
