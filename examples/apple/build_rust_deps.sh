#!/bin/bash

set -e

PATH=$PATH:$HOME/.cargo/bin

if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  # Assume we're in Xcode, which means we're probably cross-compiling.
  # In this case, we need to add an extra library search path for build scripts and proc-macros,
  # which run on the host instead of the target.
  # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

cargo build -Zbuild-std \
    --target aarch64-apple-tvos-sim \
    --target aarch64-apple-tvos \
    --target aarch64-apple-visionos \
    --target aarch64-apple-visionos-sim \
    --release

cargo build --target aarch64-apple-ios --target aarch64-apple-ios-sim --release
