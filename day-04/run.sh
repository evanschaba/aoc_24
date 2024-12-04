#!/bin/zsh

cargo fmt && \
cargo clippy --fix --allow-dirty --allow-staged && \
cargo run --bin part-1 -- input.txt > tmp 2>&1 && \
for file in src/bin/*.rs; do
    echo "=== $file ===" >> tmp
    head -n 1000 "$file" >> tmp
done