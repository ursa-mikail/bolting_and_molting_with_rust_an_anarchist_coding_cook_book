// src/utils/utils.rs

use std::collections::HashMap;

// A function to simulate borrowing money in Rust
pub fn borrower_status(total_money: f64) -> HashMap<String, String> {
    let mut status_map = HashMap::new();

    if total_money < 1000.0 {
        status_map.insert("Borrower 2".to_string(), "Doesn't return the money!".to_string());
    }

    if total_money == 1000.0 {
        status_map.insert("Borrower 1".to_string(), "Returned the money.".to_string());
        status_map.insert("Borrower 3".to_string(), "Returned the money.".to_string());
    }

    status_map
}
