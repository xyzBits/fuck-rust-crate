use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Sleep;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let shared_vector = Arc::new(Mutex::new(Vec::new()));
    let vector1 = shared_vector.clone();
    let vector2 = shared_vector.clone();

    let task1 = tokio::spawn(async move {
        for i in 0..3 {
            {
                let mut vector = vector1.lock().await;
                vector.push(i);
                println!("Task1: vector = {:?}", *vector);
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let task2 = tokio::spawn(async move {
        for i in 6..9 {
            {
                let mut vector = vector2.lock().await;
                if !vector.is_empty() {
                    vector[1] *= 2;
                }
                println!("Task2: vector = {:?}", *vector);
            }
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
    });

    let _ = tokio::try_join!(task1, task2);
    println!("Final vector: {:?}", *shared_vector.lock().await);
}
