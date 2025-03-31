use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use rand::Rng;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        for i in 1..=5 {
            let message = format!("Thread 1 says: Message {}", i);
            println!("{}", message);
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(rng.gen_range(500..1000)));
        }
    });

    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        while let Ok(message) = rx.recv() {
            println!("Thread 2 received: {}", message);
            let response = format!("Thread 2 responds to: {}", message);
            println!("{}", response);
            thread::sleep(Duration::from_millis(rng.gen_range(500..1000)));
        }
    });

    thread::sleep(Duration::from_secs(5));
    println!("Conversation between threads completed.");
}

/*

Thread 1 says: Message 1
Thread 2 received: Thread 1 says: Message 1
Thread 2 responds to: Thread 1 says: Message 1
Thread 1 says: Message 2
Thread 2 received: Thread 1 says: Message 2
Thread 2 responds to: Thread 1 says: Message 2
Thread 1 says: Message 3
Thread 2 received: Thread 1 says: Message 3
Thread 2 responds to: Thread 1 says: Message 3
Thread 1 says: Message 4
Thread 1 says: Message 5
Thread 2 received: Thread 1 says: Message 4
Thread 2 responds to: Thread 1 says: Message 4
Thread 2 received: Thread 1 says: Message 5
Thread 2 responds to: Thread 1 says: Message 5
Conversation between threads completed.
*/