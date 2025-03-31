use rand::Rng;
//use std::sync::Arc;
use std::thread;
//use std::sync::Mutex;
use std::time::Instant;

mod libs;
mod utils;

use crate::libs::p0::p0;
use crate::utils::util_00;

//use task_execution::libs::p0;
//use task_execution::utils::util_00;

fn main() {
    let tasks = p0::list_tasks();
    println!("Available tasks: {:?}", tasks);

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let tasks = tasks.clone();
            thread::spawn(move || {
                let task_name = tasks[rand::thread_rng().gen_range(0..tasks.len())];
                let start_time = Instant::now();
                match task_name {
                    "Task1" => p0::task1(),
                    "Task2" => p0::task2(),
                    "Task3" => p0::task3(),
                    "Task4" => p0::task4(),
                    _ => println!("Unknown task: {}", task_name),
                }
                util_00::log_execution_time(task_name, start_time);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks completed.");
}

/*
cargo check
cargo run

Available tasks: ["Task1", "Task2", "Task3", "Task4"]
Task2: Starting...
Task3: Starting...
Task1: Starting...
Task4: Starting with 6 rounds...
Task2: Starting...
Task4: Round 1 of 6
Task4: Round 2 of 6
Task4: Round 3 of 6
Task4: Round 4 of 6
Task4: Round 5 of 6
Task4: Round 6 of 6
Task4: Finished.
Task Task4: Duration: 606.555792ms
Task1: Finished.
Task Task1: Duration: 1.005085375s
Task2: Finished.
Task Task2: Duration: 4.001474375s
Task2: Finished.
Task Task2: Duration: 4.003569334s
Task3: Finished.
Task Task3: Duration: 5.004623125s
All tasks completed.
*/