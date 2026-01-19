use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // println!("{v:?}"); // This will cause a compile-time error because 'v' has been moved to the spawned thread.

    handle.join().unwrap();
}