use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;

const EMPTY: u8 = 0;
const FULL:  u8 = 1;
const TAKEN: u8 = 2;

struct Shared {
    data: UnsafeCell<u64>,
    state: AtomicU8,
}
unsafe impl Sync for Shared {} // synchronized via `ready`, i.e. we are telling the compiler, "we have manually implemented a synchronization logic", the Acquire-Release pair

fn main() {
    let s = Arc::new(Shared { data: UnsafeCell::new(0), state: AtomicU8::new(EMPTY) });

    let producer = {
        let s = Arc::clone(&s);
        thread::spawn(move || {
            unsafe { *s.data.get() = 777; }
            s.state.store(FULL, Ordering::Release);          // publish data
            while s.state.load(Ordering::Acquire) != TAKEN { // observe consumer's release
                std::hint::spin_loop();
            }
        })
    };

    let consumer = {
        let s = Arc::clone(&s);
        thread::spawn(move || {
            loop {
                // RMW: if we observe FULL, we also acquire the producer's prior writes;
                // and we release TAKEN to the producer.
                let prev = s.state.swap(TAKEN, Ordering::AcqRel);
                if prev == FULL {
                    let v = unsafe { *s.data.get() };
                    assert_eq!(v, 777);
                    break;
                }
                std::hint::spin_loop();
            }
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
    println!("Kraj programa");
}
