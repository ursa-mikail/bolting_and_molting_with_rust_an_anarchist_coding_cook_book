use std::fs::File;
use std::io::{self, Read};

// Custom error type using thiserror
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

// 1. Panic Example (Intentional Failure)
#[allow(dead_code)]
pub fn cause_panic() {
    panic!("This function panics! Welcome to Rust's chaos mode.");
}

// 2. Using Result<T, E> - File Reading Example
pub fn read_file(filename: &str) -> Result<String, MyError> {
    let mut file = File::open(filename).map_err(|_| MyError::FileNotFound(filename.to_string()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 3. Using Option<T> - Safe Division
pub fn safe_division(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // Return None instead of crashing
    } else {
        Some(a / b)
    }
}

// 4. Using `?` Operator for Error Propagation
pub fn read_file_safely(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // If error, propagates automatically
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 5. Handling Multiple Errors with Anyhow
pub fn complex_operation() -> anyhow::Result<()> {
    let file_content = read_file_safely("unknown.txt")?;
    println!("File content: {}", file_content);
    Ok(())
}


