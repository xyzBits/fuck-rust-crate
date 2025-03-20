use futures::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

#[tokio::main]
async fn main() {
    println!("Starting simple Stream example....");

    // 创建一个通道，向通道发送数据
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let mut stream = ReceiverStream::new(rx);

    // 启动一个任务，向通道发送数据
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(5000)).await; //每 500 ms 发送一个值
        }
    });

    while let Some(value) = stream.next().await {
        println!("Stream received value: {}", value);
    }

    println!("Stream finished");
}
