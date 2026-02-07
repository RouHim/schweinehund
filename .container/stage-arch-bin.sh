#!/usr/bin/env bash
#
# Description:
#   Stages the specified rust binary for the current cpu architecture to the current directory.
#   Simplified version supporting x86_64 only.
#
# Parameter:
#   $1 - Binary file name to stage
#
# Example:
#   ./stage-arch-bin.sh schweinehund
#
# # # #

if [ -n "$1" ]; then
  echo "Staging binary file: $1"
else
  echo "Binary file name to stage not supplied! First parameter is required."
  exit 1
fi

CURRENT_ARCH=$(uname -m)
echo "Current arch is: $CURRENT_ARCH"

# Only support x86_64
if [ "$CURRENT_ARCH" != "x86_64" ]; then
  echo "ERROR: Only x86_64 architecture is supported"
  exit 1
fi

# Look for x86_64-unknown-linux-musl release binary
BINARY_PATH="./x86_64-unknown-linux-musl/release/${1}"

if [ ! -f "$BINARY_PATH" ]; then
  echo "ERROR: Binary not found at $BINARY_PATH"
  exit 1
fi

echo "Found binary: $BINARY_PATH"
file "$BINARY_PATH"

# Copy and prepare binary
cp "$BINARY_PATH" "${1}"
strip "${1}" 2>/dev/null || true
chmod +x "${1}"

echo "Staged binary at: $(realpath "${1}")"
