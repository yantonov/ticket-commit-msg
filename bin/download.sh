#!/usr/bin/env sh

set -eu

SCRIPT="$(basename "$0")"
cd "$(dirname "$0")"

APP_NAME="ticket-commit-msg"

# Detect OS
case "$(uname -s)" in
  Linux*)
    OS="linux"
    ;;
  Darwin*)
    OS="macos"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    OS="windows"
    ;;
  *)
    echo "Unsupported OS: $(uname -s)"
    exit 1
    ;;
esac

REPO="yantonov/ticket-commit-msg"

# Get latest tag
LATEST_TAG=$(
  curl -fsSL "https://api.github.com/repos/${REPO}/tags" \
  | jq -r '.[0].name'
)

EXECUTABLE_FILENAME="ticket-commit-msg"
ARCHIVE_NAME="${EXECUTABLE_FILENAME}-${OS}-${LATEST_TAG}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ARCHIVE_NAME}"

echo "Latest tag: ${LATEST_TAG}"
echo "Downloading: ${DOWNLOAD_URL}"

TMP_DIR="$(mktemp -d)"
ARCHIVE_PATH="${TMP_DIR}/${EXECUTABLE_FILENAME}.tar.gz"
CHECKSUM_PATH="${ARCHIVE_PATH}.sha256"

echo $ARCHIVE_PATH

# Download archive and the checksum published next to it
curl -fL "${DOWNLOAD_URL}" -o "${ARCHIVE_PATH}"
curl -fL "${DOWNLOAD_URL}.sha256" -o "${CHECKSUM_PATH}"

# Verify before unpacking: linux and git bash carry sha256sum, macos carries
# shasum. Only the hash is compared, so the file name inside the checksum file
# does not have to match the temporary one.
if command -v sha256sum > /dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(sha256sum "${ARCHIVE_PATH}" | awk '{print $1}')"
elif command -v shasum > /dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(shasum -a 256 "${ARCHIVE_PATH}" | awk '{print $1}')"
else
  echo "Neither sha256sum nor shasum is available to verify the download"
  rm -rf "${TMP_DIR}"
  exit 1
fi

EXPECTED_CHECKSUM="$(awk '{print $1}' "${CHECKSUM_PATH}")"

if [ "${ACTUAL_CHECKSUM}" != "${EXPECTED_CHECKSUM}" ]; then
  echo "Checksum mismatch for ${ARCHIVE_NAME}"
  echo "  expected ${EXPECTED_CHECKSUM}"
  echo "  actual   ${ACTUAL_CHECKSUM}"
  rm -rf "${TMP_DIR}"
  exit 1
fi

echo "Checksum ok: ${ACTUAL_CHECKSUM}"

# Extract archive
tar -xzf "${ARCHIVE_PATH}" -C "${TMP_DIR}"

# Find binary inside extracted files
BIN_PATH="$(find "${TMP_DIR}" -type f -exec sh -c 'test -x "$1"' _ {} \; -print | head -n 1)"

if [ -z "${BIN_PATH}" ]; then
  echo "Executable ${EXECUTABLE_FILENAME} is not found in the archive ${TMP_DIR}"
  rm -rf "${TMP_DIR}"
  exit 1
fi


TARGET_DIR="${HOME}/bin"
mkdir -p "${TARGET_DIR}"

# Copy binary to the target directory
cp "${BIN_PATH}" "${TARGET_DIR}/${APP_NAME}"
chmod +x "${TARGET_DIR}/${APP_NAME}"

# Cleanup
rm -rf "${TMP_DIR}"

echo "Installed: ${TARGET_DIR}/${APP_NAME}"
