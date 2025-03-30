pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub fn error(&self, message: &str, err: &str) {
        println!("[ERROR] {}: {}", message, err);
    }
}