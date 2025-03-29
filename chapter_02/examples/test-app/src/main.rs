mod libs;
mod utils;

use libs::p0;
use utils::utils::hello;

fn main() {
    println!("{}", hello());

    println!("Welcome to 'Commitment Issues: Variables and Data Types'!");
    println!("Today, we'll explore Rust's quirky variable and data type relationships.");

    p0::explain_variables();

    let name = "Ferris 🦀";
    let age = 10;
    let height = 1.75;
    let is_ferris_cute = true;

    println!("\nMeet our star:");
    println!("- Name: {} (string)", name);
    println!("- Age: {} (int)", age);
    println!("- Height: {:.2} (float64)", height);
    println!("- Is {} cute? {} (bool)", name, is_ferris_cute);

    println!("\nBut wait, what happens if we try mixing them?");
    println!("- {} says, 'Hey, can I combine age and height?'", name);
    let result = age as f64 + height;
    println!("Result: {:.2} (age + height)", result);

    println!("\nGenerating SHA256 IDs for some favorite variables:");
    let inputs = vec![
        name.to_string(), // Convert &str to String
        age.to_string(),
        format!("{:.2}", height),
        is_ferris_cute.to_string(),
    ];
    let ids = p0::generate_sha256_ids(inputs.clone());

    for (i, id) in ids.iter().enumerate() {
        println!("- {}: {}", inputs[i], id);
    }

    const MOTTO: &str = "Keep coding and stay quirky!";
    println!("\nA constant reminder: {}", MOTTO);

    println!("\nAnd remember: Variables may have commitment issues, but constants are forever.");
}

