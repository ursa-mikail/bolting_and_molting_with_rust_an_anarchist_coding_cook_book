use std::time::Instant;

pub fn log_execution_time(task_name: &str, start: Instant) {
    let duration = start.elapsed();
    println!("Task {}: Duration: {:?}", task_name, duration);
}