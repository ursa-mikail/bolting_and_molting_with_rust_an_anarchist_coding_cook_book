// src/utils/utils.rs

// A function to demonstrate ownership transfer (borrowing and losing access)
pub fn ownership_transfer_example() {
    let x = String::from("Hello, Rust!");
    let _y = x;          // Ownership transferred!
    // println!("{}", x);  // Error: Can't use x anymore, it's been moved!
}

// A function to demonstrate mutable borrowing restrictions
pub fn mutable_borrow_example() {
    let mut s1 = String::from("Rust");
    let s2 = &mut s1; // Mutable borrow
    // s1.push_str(" is awesome!"); // Error! Can't borrow `s1` as mutable twice!
    println!("{}", s2);
}

// A function to demonstrate immutable borrowing (valid usage)
pub fn immutable_borrow_example() {
    let s1 = String::from("Rust");
    let s2 = &s1; // Immutable borrow, it's fine
    println!("{}", s2); // Fine, we can use s2
}

// Function to find the longest of two borrowed string slices
pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
