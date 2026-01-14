use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn the first producer thread
    let tx1 = tx.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("Producer 1: Message 1"),
            String::from("Producer 1: Message 2"),
            String::from("Producer 1: Message 3"),
        ];
        
        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(500)); // Simulate work
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("Producer 2: Message A"),
            String::from("Producer 2: Message B"),
            String::from("Producer 2: Message C"),
        ];
        
        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(700)); // Simulate work
        }
    });

    // Consumer thread (main thread)
    for received in rx {
        println!("Received: {}", received);
    }

    // while let Ok(received) = rx.recv() {
    //     println!("Received: {}", received);
    // }

    println!("All messages received.");
}
