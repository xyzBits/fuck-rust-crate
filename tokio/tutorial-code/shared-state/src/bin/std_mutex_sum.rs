use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    for _ in 0..100 {
        let data = data.clone();

        tokio::spawn(async move {
            let mut guard = data.lock().unwrap();
            *guard += 1;
        });
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let data = *data.lock().unwrap();
    println!("{}", data);
}
