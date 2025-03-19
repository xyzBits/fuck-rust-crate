use std::sync::Arc;
use tokio::sync::Mutex;
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let shared_counter = Arc::new(Mutex::new(0));

    let counter1 = shared_counter.clone();
    let counter2 = shared_counter.clone();

    let task1 = tokio::spawn(async move {
        for i in 0..5 {
            {
                let mut counter = counter1.lock().await;
                *counter += 1;
                println!("Task1: counter: {}", *counter);
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    let task2 = tokio::spawn(async move {
        for i in 0..5 {
            {
                let mut counter = counter2.lock().await;
                *counter += 1;
                println!("Task2: counter: {}", *counter);
            }
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        }
    });

    let _ = tokio::try_join!(task1, task2);
    println!("Final counter value: {}", *shared_counter.lock().await);
}
