# 通道
 下面的代码不能被编译，因为两个任务都需要以某种方式访问 client，而 client 没有实现 copy
 因此如果没有一些可以促进共享的代码，它将无法编译，另外，Client::set 需要 &mut self，这意味着
 需要独占的访问权限才能调用它，我们可以为每个任务打开一个连接，但这不是一个好办法，
 我们不能使用 std::sync::Mutex 因为 .await 需要在持有锁的情况下调用，
 我们可以使用 tokio::sync::Mutex 但是这样又仅允许一个进行中的请求，如果客户端 实现 pipelining
 异步互斥锁又不能充分利用链接了
 
## 消息传递

结论就是使用消息传递机制，该模式 使用一个专门的任务来管理 client 中的资源，任何希望发出请求的任务都会向
client 的任务发送一条消息，client 任务代表发送方发出请求，并将响应返回给发送方



## Tokio 的通道原语

tokio 提供了许多通道，每一种都有其对应的用途
- mpsc: 多生产者，单消费者，可以发送多个值
- oneshot: 单生产者，单消费者，只能发送一个值 
- broadcast: 多生产者，多消费者(广播)，可以发送多个值，每个消费者都能看到每一个值 
- watch: 单生产者，多消费者，可以多次发送，但不会保留历史记录，消费者仅能看到最新的值 


```rust
#[tokio::main]
async fn main() {

    // 创建一个最大容量为 32 的通道
    // tokio::sync::mpsc::channel 多生产者，单消费者
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(32);


    let tx2 = tx.clone();


    tokio::spawn(async move {
        tx.send("sending from first handle").await.expect("tx send failed");
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.expect("tx2 send failed");
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {:?}", message);
    }
}

```


## 如何使用通道来管理 redis 连接，使得多个发送者可以向一个 client 发送请求