#!/bin/bash
curl --proto '=https' --tlsv1.2 -fL https://sh.rustup.rs | sh -s -- -y

source "$HOME/.cargo/env"

rustup target add wasm32-unknown-unknown

cargo install dioxus-cli --locked

export PATH="$HOME/.cargo/bin:$PATH"

dx build --release