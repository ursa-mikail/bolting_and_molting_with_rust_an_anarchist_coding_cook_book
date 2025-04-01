mod utils; // Importing the utils module

fn main() {
    println!("📖 Chapter 11: Macros - When Functions Just Aren't Enough");
    println!("🔮 Meta Mayhem: Macros and Other Dark Arts");

    // Using a macro from utils
    simple_macro!();

    // Using a function that demonstrates declarative macros
    utils::demonstrate_macros();

    // Using procedural macros (requires external crates)
    #[cfg(feature = "use_proc_macros")]
    utils::demonstrate_proc_macros();

    utils::demonstrate_proc_macros_external();

}

