// src/main.rs
mod libs;
mod utils;

use crate::utils::utils::borrower_status; // Import the function

fn main() {
    libs::p0::explain_borrowing();

    // Simulate borrowing money scenario
    let mut total_money = 1000.0; // Total available money for borrowing

    let borrower1 = &mut total_money; // Borrower 1 borrows money and returns it
    println!("Borrower 1 borrowed: 200, Remaining: {}", borrower1);
    
    // Borrower 2 does not return the money (problematic borrower)
    let borrower2 = &mut total_money; // Borrower 2 borrows money but never returns it
    println!("Borrower 2 borrowed: 300, Remaining: {}", borrower2);
    
    // Borrower 3 returns money
    let borrower3 = &mut total_money; // Borrower 3 borrows and returns money
    println!("Borrower 3 borrowed: 100, Remaining: {}", borrower3);

    // Check the status of borrowers
    let borrower_status = borrower_status(total_money); // Call the imported function
    for (borrower, status) in borrower_status {
        println!("{}: {}", borrower, status);
    }
}

/*
In Rust, borrowing allows you to lend data temporarily. However, the borrow must be returned or managed carefully.
Borrower 1 borrowed: 200, Remaining: 1000
Borrower 2 borrowed: 300, Remaining: 1000
Borrower 3 borrowed: 100, Remaining: 1000
Borrower 1: Returned the money.
Borrower 3: Returned the money.
*/
