#!/bin/bash

set -e

PATH=$PATH:$HOME/.cargo/bin

POSITIONAL_ARGS=()

# cargo clean
# rm -rf build

# ./build_rust_deps.sh --platform ${PLATFORM_DISPLAY_NAME:?} --sdk-root ${SDKROOT:?} --framework-search-paths "${FRAMEWORK_SEARCH_PATHS:?}" --header-search-paths "${HEADER_SEARCH_PATHS:?}" --gcc-preprocessor-definitions "${GCC_PREPROCESSOR_DEFINITIONS:-}" --configuration ${CONFIGURATION:?} ${FORCE_COLOR} ${ARCHS:?}

# TODO: x86_64 crashes
# xcodebuild ONLY_ACTIVE_ARCH=NO ARCHS=x86_64 -scheme coreaudio-ios-example -configuration Debug -derivedDataPath build -sdk iphonesimulator

# WORKS
# xcodebuild ONLY_ACTIVE_ARCH=NO ARCHS=arm64 -scheme coreaudio-ios-example -configuration Debug -derivedDataPath build -sdk iphoneos

while [[ $# -gt 0 ]]; do
  case $1 in
    -p|--platform)
      PLATFORM="$2"
      shift # past argument
      shift # past value
      ;;
    --root)
      SRCROOT="$2"
      shift # past argument
      shift # past value
      ;;
    -s|--sdk-root)
      SDKROOT="$2"
      shift # past argument
      shift # past value
      ;;
    --framework-search-paths)
      FRAMEWORK_SEARCH_PATHS="$2"
      shift # past argument
      shift # past value
      ;;
    --header-search-paths)
      HEADER_SEARCH_PATHS="$2"
      shift # past argument
      shift # past value
      ;;
    --gcc-preprocessor-definitions)
      GCC_PREPROCESSOR_DEFINITIONS="$2"
      shift # past argument
      shift # past value
      ;;
    --configuration)
      CONFIGURATION="$2"
      shift # past argument
      shift # past value
      ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1
      ;;
    *)
      POSITIONAL_ARGS+=("$1") # save positional arg
      shift # past argument
      ;;
  esac
done

ARCH=$POSITIONAL_ARGS

echo "SRCROOT $SRCROOT"
echo "PLATFORM $PLATFORM"
echo "SDKROOT $SDKROOT"
echo "FRAMEWORK_SEARCH_PATHS $FRAMEWORK_SEARCH_PATHS"
echo "HEADER_SEARCH_PATHS $HEADER_SEARCH_PATHS"
echo "GCC_PREPROCESSOR_DEFINITIONS $GCC_PREPROCESSOR_DEFINITIONS"
echo "CONFIGURATION $CONFIGURATION"
echo "ARCH $ARCH"
echo "POSITIONAL_ARGS $POSITIONAL_ARGS"

PREFIX="aarch64"
SUFFIX=$([ "$ARCH" = "Simulator" ] && echo "-sim" || echo "")
PREFIX=$([ "$ARCH" = "arm64" ] && echo "aarch64" || echo $PREFIX)

platform=$(tr '[:upper:]' '[:lower:]' <<<"$PLATFORM")
target="${PREFIX}-apple-${platform}${SUFFIX}"

echo "TRIPLE $target"
COMMAND=$([ "$PLATFORM" = "visionOS" ] && echo "cargo +nightly build -Zbuild-std" || echo "cargo build")

if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  # Assume we're in Xcode, which means we're probably cross-compiling.
  # In this case, we need to add an extra library search path for build scripts and proc-macros,
  # which run on the host instead of the target.
  # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

$COMMAND --target $target