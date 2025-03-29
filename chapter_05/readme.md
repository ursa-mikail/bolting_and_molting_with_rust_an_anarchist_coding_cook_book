# Rust Borrowing Example

This project demonstrates Rust's borrowing system with the analogy of lending and borrowing money. The program simulates some borrowers returning the money, while others don't, showcasing how Rust handles ownership and borrowing.

### Key Concepts:
- **Ownership**: In Rust, you own data, and only one variable can own it at a time.
- **Borrowing**: Borrowing allows you to lend data without taking ownership, either immutably or mutably.
- **Borrow Checker**: Rust’s borrow checker ensures that data is not accessed after being borrowed or used concurrently in an unsafe way.

### Functions:
- `borrower_status`: Simulates the status of borrowers, whether they return the borrowed money or not.

### How it works:
- **Borrower 1** borrows and returns money.
- **Borrower 2** borrows but doesn't return the money (problematic).
- **Borrower 3** borrows and returns money.

