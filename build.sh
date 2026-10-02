#!/bin/bash
rustup target add wasm32-unknown-unknown

curl -fsSL https://dioxuslabs.com/install.sh | bash

export PATH="$HOME/.cargo/bin:$PATH"

dx build --release