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

TARGET_DEVICE_PLATFORM_NAME=xrsimulator
echo $TARGET_DEVICE_PLATFORM_NAME
echo SDK ROOT IS: $SDKROOT
cargo +nightly build -Zbuild-std \
    --target aarch64-apple-tvos-sim \
    --target aarch64-apple-tvos \
    --target aarch64-apple-visionos \
    --target aarch64-apple-visionos-sim \
    --release

cargo build --target aarch64-apple-ios --target aarch64-apple-ios-sim --release

## Detect which platform we're building for based on SDKROOT
## SDKROOT is set by Xcode and contains the SDK path (e.g., iphoneos, iphonesimulator, appletvos, appletvsimulator)
#case "${TARGET_DEVICE_PLATFORM_NAME:-}" in
#  *Appletvos*|*AppleTVSimulator*)
#    # Build for tvOS (requires nightly toolchain and -Zbuild-std since tvOS is a Tier 3 target)
#    cargo +nightly build -Zbuild-std --target aarch64-apple-tvos-sim --target aarch64-apple-tvos --release
#    ;;
#  *XROs*|*XRSimulator*|*xrsimulator*|*rxos*)
#    # Build for tvOS (requires nightly toolchain and -Zbuild-std since tvOS is a Tier 3 target)
#    cargo +nightly build -Zbuild-std --target aarch64-apple-visionos-sim --target aarch64-apple-visionos --release
#    ;;
#  *)
#    # Default: Build for iOS (stable toolchain)
#    # Device (arm64)
#    cargo build --target aarch64-apple-ios --release
#    # Simulator (arm64)
#    cargo build --target aarch64-apple-ios-sim --release
#    ;;
#esac
