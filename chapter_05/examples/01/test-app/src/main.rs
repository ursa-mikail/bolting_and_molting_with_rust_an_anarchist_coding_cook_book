mod libs;
mod utils;

use crate::utils::utils::{ownership_transfer_example, mutable_borrow_example, immutable_borrow_example, longest};

fn main() {
    println!("--- Ownership Transfer Example ---");
    ownership_transfer_example();

    println!("\n--- Mutable Borrow Example ---");
    mutable_borrow_example();

    println!("\n--- Immutable Borrow Example ---");
    immutable_borrow_example();

    println!("\n--- Longest String Example ---");
    let string1 = String::from("Rustacean");
    let string2 = String::from("Ferris the Crab");

    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
}

/*
--- Ownership Transfer Example ---

--- Mutable Borrow Example ---
Rust

--- Immutable Borrow Example ---
Rust

--- Longest String Example ---
The longest string is: Ferris the Crab
*/