use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc<Mutex<T>> for safe, concurrent access
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Create 10 threads to increment the counter
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc for shared ownership
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the Mutex
            *num += 1; // Increment the counter
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final result
    println!("Result: {}", *counter.lock().unwrap());
}
