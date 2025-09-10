use tokio::time::{sleep, Duration};

// Async/await concurrency example

pub async fn main() {
    let handles = (0..5).map(|i| {
        tokio::spawn(async move {
            println!("Async task {} is running", i);
            sleep(Duration::from_millis(500)).await;
            println!("Async task {} is done", i);
        })
    });
    for handle in handles {
        handle.await.unwrap();
    }
    println!("All async tasks have finished.");
}

