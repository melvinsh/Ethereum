#!/bin/bash
set -e

cd "$(dirname "$0")"
cargo build --release

# Run with any arguments passed to the script
./target/release/eth-wallet-generator "$@" 