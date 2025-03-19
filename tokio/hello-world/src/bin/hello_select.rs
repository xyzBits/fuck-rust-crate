#[tokio::main]
async fn main() {
    println!("Starting select example....");

    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_millis(1000)) => {
            println!("Task1 completed after 1 second successfully!");
        },

        _ = tokio::time::sleep(std::time::Duration::from_millis(2000)) => {
            println!("Task2 completed after 2 seconds successfully!");
        }
    }

    println!("Select finished");
}
