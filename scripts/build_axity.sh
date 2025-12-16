#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."
cargo build --release
echo "Built: $(pwd)/target/release/axity"
