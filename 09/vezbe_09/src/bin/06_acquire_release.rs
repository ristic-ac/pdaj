use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

struct Shared {
    data: UnsafeCell<u64>,
    ready: AtomicBool,
}
unsafe impl Sync for Shared {} // synchronized via `ready`, i.e. we are telling the compiler, "we have manually implemented a synchronization logic", the Acquire-Release pair

fn main() {
    let s = Arc::new(Shared { data: UnsafeCell::new(0), ready: AtomicBool::new(false) });

    let producer = {
        let s = Arc::clone(&s);
        thread::spawn(move || {
            unsafe { *s.data.get() = 123; }          // ordinary write
            s.ready.store(true, Ordering::Release);  // publish
        })
    };

    let consumer = {
        let s = Arc::clone(&s);
        thread::spawn(move || {
            while !s.ready.load(Ordering::Acquire) { std::hint::spin_loop(); }
            // A spin loop hint is a specialized hardware instruction, such as PAUSE on x86, that informs the CPU a thread is currently busy-waiting in a tight loop, allowing the processor to reduce power consumption, e.g. by not executing speculations.
            let v = unsafe { *s.data.get() };
            assert_eq!(v, 123);
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
    println!("Kraj programa");
}
