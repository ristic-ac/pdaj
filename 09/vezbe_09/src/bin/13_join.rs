use tokio;
use tokio::time::{sleep, Duration};

async fn fetch_user() -> String {
    sleep(Duration::from_secs(1)).await;
    "User: Marco".to_string()
}

async fn fetch_orders() -> Vec<String> {
    sleep(Duration::from_secs(2)).await;
    vec!["Order 1".to_string(), "Order 2".to_string()]
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    let (user, orders) = tokio::join!(
        fetch_user(),
        fetch_orders()
    );

    let duration = start.elapsed();

    println!("Data: {}, {:?}", user, orders);
    println!("Total time (estimated ~2s): {:?}", duration);
}