# Rust with Python Example

This project demonstrates how to integrate Rust with Python using PyO3 and Maturin. It shows various ways to call Rust code from Python, including:

- Basic function calls
- String processing
- NumPy array operations
- Custom classes defined in Rust

## Directory Structure

```
.
├── build.sh         # Build script
├── python/
│   └── example.py   # Python example script
├── rust/
│   ├── Cargo.toml   # Rust package configuration
│   └── src/
│       └── lib.rs   # Rust library code
├── rust_with_python/ # Python package directory
└── setup.py        # Python setup script
```

## Prerequisites

- Python 3.7 or higher
- Rust 1.48 or higher
- pip
- NumPy

## Building and Running

1. Make the build script executable:
   ```
   chmod +x build.sh
   ```

2. Run the build script:
   ```
   ./build.sh
   ```

This will:
- Install the required Python dependencies
- Build the Rust extension
- Run the example Python script

## How It Works

The integration works using PyO3, a Rust library that provides bindings between Python and Rust. The Rust code is compiled into a native Python extension module.

### Features Demonstrated

1. **Basic Function Calls**: The `fibonacci` function shows how to expose a simple Rust function to Python.

2. **String Operations**: The `reverse_string` function demonstrates handling Python strings in Rust.

3. **NumPy Integration**: The `add_vectors` and `matrix_multiply` functions show how to work with NumPy arrays in Rust using the `numpy` and `ndarray` crates.

4. **Custom Classes**: The `Point` class demonstrates how to create Python classes in Rust, complete with methods and attributes.

## Performance Comparison

The example includes a performance comparison between Python and Rust implementations of the Fibonacci function, demonstrating the potential speed improvements.
