#!/bin/bash

# build the wasm
cargo build --target wasm32-unknown-unknown --release

# copy the wasm to the docs folder
cp target/wasm32-unknown-unknown/release/autocel.wasm docs/
