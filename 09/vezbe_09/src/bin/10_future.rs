use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

struct SharedState {
    ready: bool,
    waker: Option<Waker>,
}

struct SimpleFuture {
    state: Arc<Mutex<SharedState>>,
}

impl Future for SimpleFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.ready {
            Poll::Ready("Podaci su spremni!".to_string())
        } else {
            // Reregister waker because of the new context cx
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(SharedState { ready: false, waker: None }));
    let future_result = SimpleFuture { state: state.clone() };

    // Simulating some work done in the background which triggers polling
    thread::spawn(move || {
        // Wait for some time to simulate data transfer
        thread::sleep(Duration::from_secs(3));
        let mut s = state.lock().unwrap();
        s.ready = true;
        
        // Notify waker
        if let Some(waker) = s.waker.take() {
            waker.wake();
        }
    });

    println!("Čekam na Future...");
    let poruka = future_result.await;
    println!("Dobijeno: {}", poruka);
}