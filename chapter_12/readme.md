cd examples
cargo new test-app

cd src
cargo clean
cargo update
cargo build

cargo run

/*
Main function called
Library function called
Utility function called
*/



Unsafe Rust is like juggling chainsaws—cool if you know what you're doing, disastrous if you don't.

Use it for performance boosts, FFI (Foreign Function Interface), and low-level memory control—but tread carefully.

Rust’s safety guarantees? Gone. You’re on your own now.

Key Topics:
🔥 What Unsafe Rust Unlocks – Raw pointers, mutable statics, unions, and more.
🔥 Why It’s Dangerous – Buffer overflows, data races, and "Oops, I corrupted my entire program."
🔥 Best Practices – Only use unsafe where necessary, wrap it in safe abstractions, and comment why you're using it.

⚠️ Warning: If you misuse unsafe Rust, future-you will have some very unsafe words for past-you.


Rust discourages creating shared references (&COUNTER) to static mut variables because it can lead to data races in concurrent code. To fix this, you have a few options:

Solution 1: Use unsafe Block (If Single-Threaded)
If you are sure that COUNTER will not be accessed concurrently, wrap the access in an unsafe block:

```
static mut COUNTER: i32 = 0;

pub unsafe fn modify_static_variable() {
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
```

Solution 2: Use AtomicI32 for Thread Safety (Preferred)
If multiple threads will modify COUNTER, use AtomicI32 instead of static mut. This makes operations atomic and eliminates the need for unsafe.

```
use std::sync::atomic::{AtomicI32, Ordering};

static COUNTER: AtomicI32 = AtomicI32::new(0);

pub fn modify_static_variable() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Counter: {}", COUNTER.load(Ordering::SeqCst));
}
```

Why is this better?

AtomicI32 allows safe concurrent access without unsafe.

fetch_add(1, Ordering::SeqCst) increments COUNTER atomically.

Ordering::SeqCst ensures memory ordering is respected across threads.

Solution 3: Use Mutex for Safer Access
If you need more flexibility (e.g., modifying a struct), use a Mutex:

```
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
}

pub fn modify_static_variable() {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    println!("Counter: {}", *counter);
}
```

Why use Mutex?

Ensures only one thread modifies COUNTER at a time.

No need for unsafe.

Which Solution Should You Use?
Scenario	Solution
Single-threaded use	Solution 1 (Unsafe Block)
Multi-threaded integer counter	Solution 2 (AtomicI32)
More complex shared data	Solution 3 (Mutex)
Final Recommendation: If this is a simple counter, use AtomicI32 (Solution 2) to keep things safe and fast. 🚀