#!/usr/bin/env bash

set -e

sudo apt update && sudo apt install pkg-config

rustup target add wasm32-unknown-unknown
cargo install --locked trunk