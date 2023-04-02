#!/usr/bin/env bash

set -e

rustup target add wasm32-unknown-unknown
cargo install --locked trunk