#!/usr/bin/env bash
set -euo pipefail

echo "Rust:"
rustc --version
echo "Cargo:"
cargo --version

echo "Running benches..."
cargo bench --all-features
