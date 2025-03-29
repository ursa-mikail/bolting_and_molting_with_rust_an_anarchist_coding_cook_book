use sha2::{Sha256, Digest};
use hex::encode;

pub fn explain_variables() {
    println!("Commitment Issues: Why can't variables and types just get along?");
    println!("- A string says, 'I'm here for the long haul, but only with text.'");
    println!("- An int replies, 'Numbers are my thing, and I prefer no decimal drama.'");
    println!("- A float counters, 'Decimals? Drama? No, it's elegance.'");
    println!("- A bool smirks, 'True or false, I'm all about commitment.'");
}

pub fn generate_sha256_ids(inputs: Vec<String>) -> Vec<String> {
    let mut ids = vec![];
    for input in inputs {
        let hash = Sha256::digest(input.as_bytes());
        ids.push(encode(hash));
    }
    ids
}
