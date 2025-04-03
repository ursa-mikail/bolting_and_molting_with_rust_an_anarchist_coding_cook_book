#!/bin/bash
# chmod +x build.sh
# ./build.sh
set -e

echo "=== Starting build process ==="

# Step 1: Build Rust library first
echo "Building Rust library..."
cd rust
cargo build --release
echo "Rust library build completed."
cd ..

# Step 2: Build Go code that depends on the Rust library
echo "Building Go executable..."
cd golang
export LIBRARY_PATH=../rust/target/release
export LD_LIBRARY_PATH=../rust/target/release
go build -o rust_go_example main.go
echo "Go build completed."
cd ..

# Step 3: Run the Go example
echo "=== Build process finished ==="
echo "Running Go example:"
cd golang
./rust_go_example

echo "To run the Go example again: cd golang && ./rust_go_example"