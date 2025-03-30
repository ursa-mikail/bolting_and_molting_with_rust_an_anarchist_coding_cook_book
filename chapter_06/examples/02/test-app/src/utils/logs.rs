use std::panic;

// === Mission Logs: Panic and Recovery ===
pub fn process_log(log: &str) {
    let result = panic::catch_unwind(|| {
        println!("Processing log: {}", log);

        // Simulate a critical failure
        if log == "Hull breach detected" {
            panic!("🚨 Critical failure: Hull integrity compromised!");
        }
    });

    if let Err(err) = result {
        println!("⚠️ Recovered from catastrophic error: {:?}", err);
    }
}
