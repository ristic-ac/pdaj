use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::runtime::Builder;

/// Represents the desugared state machine of the `hello()` async function.
struct HelloFuture {
    state: State,
}

enum State {
    Start,
    Done,
}

impl Future for HelloFuture {
    type Output = &'static str;

    /// The executor calls this method to drive the future to completion.
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            State::Start => {
                // Transition state and return the result
                self.state = State::Done;
                Poll::Ready("Async hello world!")
            }
            State::Done => {
                panic!("Future polled after completion");
            }
        }
    }
}

/// Equivalent to: async fn hello() -> &'static str { ... }
fn hello() -> HelloFuture {
    HelloFuture { state: State::Start }
}

fn main() {
    // 1. Manually build the Tokio multi-threaded runtime.
    // This replaces the logic inside the #[tokio::main] macro.
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    // 2. Entry point into the asynchronous context.
    // block_on takes ownership of the future and drives it to completion.
    rt.block_on(async {
        println!("Tokio runtime ready.");
        
        // 3. Manually polling our custom future.
        // Inside an async block, .await handles the poll loop.
        let message = hello().await;
        
        println!("{}", message);
    });
}