#!/bin/sh

cd "$(dirname "$0")"

cd ..

EXECUTABLE_NAME="ticket-commit-msg"

TARGET="$(pwd)/target/debug/${EXECUTABLE_NAME}"

cargo build

echo "binary file is here: ${TARGET}"
