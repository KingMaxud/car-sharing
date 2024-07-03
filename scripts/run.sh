#!/usr/bin/env bash

if [[ "$ENV" = "TEST" ]]; then
    echo "Running release version of server for integration tests"
    ./target/release/messenger_rocket
else
    echo "Building Axum server"
    cargo run
fi
