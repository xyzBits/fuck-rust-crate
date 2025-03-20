use futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("Starting Stream with transformations....");

    let stream = tokio_stream::iter(0..10)
        .filter(|&x| async move { x % 2 == 0 })
        .map(|x| async move { x * 2 });

    tokio::pin!(stream);

    while let Some(value) = stream.next().await {
        println!("Processed value: {:?}", value.await);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("Stream finished");
}
