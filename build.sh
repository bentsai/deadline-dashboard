#!/usr/bin/env bash
set -euo pipefail

if [ ! -d wit/deps ]; then
    echo "Fetching wasi-http WIT deps..."
    curl -sSL -o /tmp/wasi-http.tar.gz \
      'https://github.com/WebAssembly/wasi-http/archive/refs/tags/v0.2.0.tar.gz'
    tar -xzf /tmp/wasi-http.tar.gz -C /tmp
    cp /tmp/wasi-http-0.2.0/wit/*.wit wit/
    cp -r /tmp/wasi-http-0.2.0/wit/deps wit/
fi

cargo build --release --target wasm32-wasip2

artifact="target/wasm32-wasip2/release/deadline_dashboard.wasm"
echo "Built $artifact ($(wc -c < "$artifact") bytes)"
