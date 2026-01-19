use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let t1 = {
        let c = Arc::clone(&counter);
        thread::spawn(move || for _ in 0..1_000_000 { c.fetch_add(1, Ordering::Relaxed); })
    };
    let t2 = {
        let c = Arc::clone(&counter);
        thread::spawn(move || for _ in 0..1_000_000 { c.fetch_add(1, Ordering::Relaxed); })
    };

    t1.join().unwrap();
    t2.join().unwrap();

    assert_eq!(counter.load(Ordering::Relaxed), 2_000_000);
    println!("Kraj programa");
}
