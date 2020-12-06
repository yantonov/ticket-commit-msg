#!/bin/sh

cd "$(dirname "$0")"

cd ..

EXECUTABLE_NAME="ticket-commit-msg"

TARGET="$(pwd)/target/release/${EXECUTABLE_NAME}"

cargo build --release

echo "binary file is here: ${TARGET}"
