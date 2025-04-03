mod utils;
use crate::utils::utils::call_c_functions;

fn main() {
    println!("Rust calling C functions...");
    call_c_functions();
}

/*
Rust calling C functions...
Absolute value of -42 is 42
Hello from Rust!
*/