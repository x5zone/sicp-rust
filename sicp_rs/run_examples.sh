#!/bin/bash

for example in $(find examples -name "*.rs" | sed 's|examples/||;s|.rs$||'); do
    echo ""
    echo "Running example: $example"
    cargo run --example $example
done
