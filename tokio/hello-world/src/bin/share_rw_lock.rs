use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let shared_counter = Arc::new(RwLock::new(0));

    let counter1 = shared_counter.clone();
    let counter2 = shared_counter.clone();

    let task1 = tokio::spawn(async move {
        for i in 0..5 {
            {
                let mut counter = counter1.write().await;
                *counter += 1;
                println!("Task1: write counter =  {}", *counter);
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    let task2 = tokio::spawn(async move {
        for _ in 0..5 {
            let counter = counter2.read().await;
            println!("Task2: read counter =  {}", *counter);
        }

        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    });

    let _ = tokio::try_join!(task1, task2);
    println!("Final counter value: {}", *shared_counter.read().await);
}
