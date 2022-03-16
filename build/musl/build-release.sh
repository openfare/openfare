#!/bin/bash
set -euo pipefail

echo "Building static binaries using ekidd/rust-musl-builder"
podman build -t build-"$1"-image -f build/musl/Containerfile .
podman run -it --name build-"$1" build-"$1"-image
podman cp build-"$1":/home/rust/src/target/x86_64-unknown-linux-musl/release/"$1" "$2"
podman rm build-"$1"
podman rmi build-"$1"-image
