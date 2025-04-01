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


📌 Running the Code
Run the program normally:

cargo run
If using procedural macros, enable the feature:
cargo run --features "use_proc_macros"

``` chapter_11/examples/test-app/src/ % cargo run --features "use_proc_macros"

📖 Chapter 11: Macros - When Functions Just Aren't Enough
🔮 Meta Mayhem: Macros and Other Dark Arts
✨ This is a simple macro! ✨
🔹 Running macro examples...
Repeated: Hello, Rust! Hello, Rust!
Function `hello_macro` generated!
✅ Procedural macro applied to struct `Example`.
🛠 Custom Debug Output: ExampleStruct { Custom Debug! }
```

```

│── proc_macros/  # New procedural macro crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
│── test-app/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── libs/
│   │   │   └── mod.rs
│   │   ├── main.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       └── utils.rs
```
