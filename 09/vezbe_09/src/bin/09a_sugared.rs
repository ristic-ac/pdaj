use tokio;

#[tokio::main]
async fn main() {
    println!("Tokio runtime ready.");
    let message = hello().await;
    println!("{}", message);
}

async fn hello() -> &'static str {
    "Async hello world!"
}