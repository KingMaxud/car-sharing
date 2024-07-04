#!/bin/bash
set -e

# Running Unit Tests
cargo test -- --test-threads 1 --nocapture
