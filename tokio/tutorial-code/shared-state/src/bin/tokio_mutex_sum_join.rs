use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let tasks = (0..100)
        .map(|_| {
            let data = data.clone();
            tokio::spawn(async move {
                let mut guard = data.lock().await;

                *guard += 1;
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;

    let guard = data.lock().await;
    println!("{}", *guard);
}
