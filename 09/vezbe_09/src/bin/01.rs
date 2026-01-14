use std::thread;
use std::time::Duration;

fn main() {
    let data = vec![10, 20, 30, 40, 50];

    // Spawn a new thread to process data
    let handle = thread::spawn(move || {
        for value in data {
            println!("Processing value {} in the spawned thread.", value);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Main thread processes its own task
    for i in 1..6 {
        println!("Performing task {} in the main thread.", i);
        thread::sleep(Duration::from_millis(100));
    }

    // Ensure the spawned thread finishes before exiting
    handle.join().unwrap();

    println!("All tasks complete!");
}
