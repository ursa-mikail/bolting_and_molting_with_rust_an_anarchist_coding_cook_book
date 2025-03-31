use rand::Rng;
use std::thread;
use std::time::Duration;

pub fn task1() {
    println!("Task1: Starting...");
    thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(1..=2)));
    println!("Task1: Finished.");
}

pub fn task2() {
    println!("Task2: Starting...");
    thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(2..=4)));
    println!("Task2: Finished.");
}

pub fn task3() {
    println!("Task3: Starting...");
    thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(3..=6)));
    println!("Task3: Finished.");
}

pub fn task4() {
    let rounds = rand::thread_rng().gen_range(1..=500);
    println!("Task4: Starting with {} rounds...", rounds);
    for i in 1..=rounds {
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(50..150)));
        println!("Task4: Round {} of {}", i, rounds);
    }
    println!("Task4: Finished.");
}

pub fn list_tasks() -> Vec<&'static str> {
    vec!["Task1", "Task2", "Task3", "Task4"]
}
