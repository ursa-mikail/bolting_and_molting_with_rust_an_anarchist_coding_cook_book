#!/bin/bash
# chmod +x build.sh
# ./build.sh
#!/bin/bash
#!/bin/bash
set -e

echo "=== Starting build process ==="

# Step 1: Build Rust library
echo "Building Rust library..."
cd rust
cargo build --release

# Create include directory if it doesn't exist
mkdir -p include
echo "Rust library build completed."
cd ..

# Step 2: Build C program (with fixed linking for macOS ARM64)
echo "Building C program..."
gcc -Wall -Wextra -g -o rust_c_example c/main.c \
    -Lrust/target/release \
    -lrust_with_c \
    -Wl,-rpath,./rust/target/release \
    -Wl,-dead_strip_dylibs

# On macOS, nm can help debug which symbols are available
echo "Checking available symbols in the Rust library:"
nm -g rust/target/release/librust_with_c.dylib | grep -E 'add|uppercase|distance'

echo "C program build completed."

# Step 3: Run the example
echo "=== Build process finished ==="
echo "Running C example:"
DYLD_PRINT_LIBRARIES=1 ./rust_c_example

echo "To run the example again: ./rust_c_example"