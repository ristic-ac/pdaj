use tokio::runtime::Builder;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use tokio::time::{sleep, Duration};

fn main() {
    // Shared atomic counter to assign unique indices to threads
    let thread_counter = Arc::new(AtomicUsize::new(0));

    // Configure a multi-threaded runtime manually
    let rt = Builder::new_multi_thread()
        .worker_threads(4) // Allocate exactly 4 threads to the pool
        .thread_name("tokio-pool") // Base name for the threads
        .on_thread_start(move || {
            // This block runs once when each worker thread starts
            let id = thread_counter.fetch_add(1, Ordering::SeqCst);
            println!("Starting worker thread index: {}", id);
        })
        .enable_all() // Enable I/O and time drivers
        .build()
        .expect("Failed to build Tokio runtime");

    // Execute the async entry point
    rt.block_on(async {
        let mut handles = vec![];

        for i in 0..10 {
            // Spawn tasks to be distributed across the 4 threads
            let handle = tokio::spawn(async move {
                let current = thread::current();
                let name = current.name().unwrap_or("unknown");
                let id = current.id();
                
                println!("Task {} running on thread [{}] (OS ID: {:?})", i, name, id);
                
                // Simulate asynchronous work
                sleep(Duration::from_millis(50)).await;
            });
            handles.push(handle);
        }

        // Wait for all spawned tasks to complete
        for handle in handles {
            let _ = handle.await;
        }
    });
}