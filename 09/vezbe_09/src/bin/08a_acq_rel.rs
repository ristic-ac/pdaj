use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut detected_failure = 0;
    let iterations = 1_000_000; // Increased iterations

    for i in 0..iterations {
        let x = Arc::new(AtomicUsize::new(0));
        let y = Arc::new(AtomicUsize::new(0));
        let gate = Arc::new(AtomicUsize::new(0)); // To sync thread start

        // Make Arcs for t1
        let x1 = Arc::clone(&x);
        let y1 = Arc::clone(&y);
        let gate1 = Arc::clone(&gate);
        // Make Arcs for t2
        let x2 = Arc::clone(&x);
        let y2 = Arc::clone(&y);
        let gate2 = Arc::clone(&gate);

        let t1 = thread::spawn(move || {
            while gate1.load(Ordering::Acquire) == 0 {
                // Wait for start signal
            } 
            x1.store(1, Ordering::Release);
            y1.load(Ordering::Acquire)
        });

        let t2 = thread::spawn(move || {
            while gate2.load(Ordering::Acquire) == 0 {
                // Wait for start signal
            }   
            y2.store(1, Ordering::Release);
            x2.load(Ordering::Acquire)
        });

        // Fire the starting pistol!
        gate.store(1, Ordering::SeqCst);

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();

        if r1 == 0 && r2 == 0 {
            detected_failure += 1;
            println!("Iteration {}: Both read 0!", i);
        }
    }

    println!("Finished {} iterations.", iterations);
    println!("Total reorderings detected: {}", detected_failure);
    
    if detected_failure == 0 {
        println!("Note: Your CPU (likely x86) is very strict. Try 10 million iterations or an ARM processor!");
    }
}