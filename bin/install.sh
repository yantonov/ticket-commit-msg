#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

cargo build --release

TARGET="${HOME}/bin/ticket-commit-msg"
if [ -f "${TARGET}" ] || [ -L "${TARGET}" ]; then
    echo "Remove old file ${TARGET} for the hook"
    rm "${TARGET}"
fi

echo "Create symlink ${TARGET} for the hook"
cp $(pwd)/target/release/ticket-commit-msg ${TARGET}

TARGET="${HOME}/bin/ticket-commit-msg-install"
if [ -L "${TARGET}" ]; then
    echo "Remove old symlink ${TARGET} for install script"
    rm "${TARGET}"
fi

echo "Create symlink ${TARGET} for install script"
ln -s $(pwd)/install ${TARGET}

echo 'Done'
