#[macro_export] 
macro_rules! simple_macro {
    () => {
        println!("✨ This is a simple macro! ✨");
    };
}

pub fn demonstrate_macros() {
    println!("🔹 Running macro examples...");

    // Using a macro to create repetitive structures
    macro_rules! repeat_twice {
        ($val:expr) => {
            println!("Repeated: {} {}", $val, $val);
        };
    }

    repeat_twice!("Hello, Rust!");

    // Using a macro for code generation
    macro_rules! generate_function {
        ($name:ident) => {
            pub fn $name() {
                println!("Function `{}` generated!", stringify!($name));
            }
        };
    }

    generate_function!(hello_macro);
    hello_macro();
}

#[cfg(feature = "use_proc_macros")]
pub fn demonstrate_proc_macros() {
    use proc_macros::CustomDebug; // Example procedural macro

    #[derive(CustomDebug)]
    struct Example;

    println!("✅ Procedural macro applied to struct `Example`.");
}


use proc_macros::CustomDebug;

#[derive(CustomDebug)]
pub struct ExampleStruct {
    value: i32,
}

pub fn demonstrate_proc_macros_external() {
    let instance = ExampleStruct { value: 42 };
    println!("🛠 Custom Debug Output: {:?}", instance);
}

/*
1. Declarative Macros (macro_rules!)

simple_macro!() prints a message.

repeat_twice!($val) takes an argument and prints it twice.

generate_function!($name) dynamically generates a function.

2. Procedural Macros (Optional)

Uses a #[cfg(feature = "use_proc_macros")] flag to conditionally compile procedural macros.

Requires an external crate like proc-macro2 or a custom procedural macro crate.
*/