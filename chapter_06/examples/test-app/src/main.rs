mod libs;
mod utils;

use utils::error_handling::*;

fn main() {
    println!("--- Handling Errors in Rust ---");

    // 1. Panic Handling Example
    // Uncomment to see it crash
    // cause_panic();

    // 2. Result<T, E> Handling
    match read_file("nonexistent.txt") {
        Ok(content) => println!("File read successfully: {}", content),
        Err(e) => println!("Error occurred: {}", e),
    }

    // 3. Option<T> Handling
    let result = safe_division(10.0, 0.0);
    match result {
        Some(value) => println!("Division result: {}", value),
        None => println!("Cannot divide by zero!"),
    }

    // 4. Propagating Errors with `?`
    match read_file_safely("nonexistent.txt") {
        Ok(content) => println!("Safe read: {}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }

    // 5. Handling Complex Errors with Anyhow
    if let Err(e) = complex_operation() {
        println!("Complex operation failed: {:?}", e);
    }

    // Explicitly using lib_function to avoid warning
    libs::lib_function();

    // Calling cause_panic but inside a safe block
    let _ = std::panic::catch_unwind(|| cause_panic());

    // Use InvalidInput by triggering a custom error
    let invalid_error = MyError::InvalidInput("This is an invalid input".to_string());
    println!("Example of custom error: {}", invalid_error);

}

/*
--- Handling Errors in Rust ---
Error occurred: File not found: nonexistent.txt
Cannot divide by zero!
Failed to read file: No such file or directory (os error 2)
Complex operation failed: No such file or directory (os error 2)
Library function called

thread 'main' panicked at src/utils/error_handling.rs:22:5:
This function panics! Welcome to Rust's chaos mode.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Example of custom error: Invalid Input: This is an invalid input
*/
