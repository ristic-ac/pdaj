use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Two shared Mutexes wrapped in Arc for thread-safe sharing
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));

    // Clone Arc handles for thread 1
    let mutex1_t1 = Arc::clone(&mutex1);
    let mutex2_t1 = Arc::clone(&mutex2);

    // Clone Arc handles for thread 2
    let mutex1_t2 = Arc::clone(&mutex1);
    let mutex2_t2 = Arc::clone(&mutex2);

    // Thread 1 locks mutex1 first, then tries to lock mutex2
    let thread1 = thread::spawn(move || {
        let lock1 = mutex1_t1.lock().unwrap();
        println!("Thread 1: Locked mutex1");

        // Simulate work
        thread::sleep(std::time::Duration::from_millis(100));

        let lock2 = mutex2_t1.lock().unwrap(); // Deadlock occurs here
        println!("Thread 1: Locked mutex2");
    });

    // Thread 2 locks mutex2 first, then tries to lock mutex1
    let thread2 = thread::spawn(move || {
        let lock2 = mutex2_t2.lock().unwrap();
        println!("Thread 2: Locked mutex2");

        // Simulate work
        thread::sleep(std::time::Duration::from_millis(100));

        let lock1 = mutex1_t2.lock().unwrap(); // Deadlock occurs here
        println!("Thread 2: Locked mutex1");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
