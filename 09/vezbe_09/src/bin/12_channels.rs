use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(v) = rx.recv().await {
        println!("Recieved: {}", v);
    }
}