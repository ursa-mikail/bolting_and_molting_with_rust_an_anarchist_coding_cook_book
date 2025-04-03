#!/bin/bash
# chmod +x build.sh
# ./build.sh
#!/bin/bash
#!/bin/bash
set -e

echo "=== Starting build process ==="

# Check if Python is installed
if ! command -v python3 &> /dev/null; then
    echo "Error: Python 3 is required but not found."
    exit 1
fi

# Check if pip is installed
if ! command -v pip3 &> /dev/null; then
    echo "Error: pip3 is required but not found."
    exit 1
fi

# Install dependencies
echo "Installing Python dependencies..."
pip3 install numpy maturin

# Create necessary directory structure for Python package
echo "Setting up Python package structure..."
mkdir -p python

# Use Maturin to build and develop the package
echo "Building and developing the Rust extension with Maturin..."
maturin develop --release

echo "=== Build process completed ==="
echo "Running example Python script:"
python3 python/example.py