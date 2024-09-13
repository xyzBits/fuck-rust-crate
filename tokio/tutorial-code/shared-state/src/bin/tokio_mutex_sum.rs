use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    for _ in 0..100 {
        let data = data.clone();

        tokio::spawn(async move {
            let mut guard = data.lock().await;

            *guard += 1;
        });
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("{:?}", data.lock().await);
}
