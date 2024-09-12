# Spawning

## 接收套接字 Accepting sockets 
Redis 服务器需要做的第一件事是接收入站的 TCP sockets，
使用 tokio::net::TcpListener 

> 大多数tokio 的类型名和 Rust 中标准库中具有相同的类型名称，只是tokio中使用的API 是 async的

一个 TcpListener 绑定到 6379 端口，然后在 loop 循环接收 sockets，每个 socket 都经过处理然后关闭
```rust
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {

    // 绑定监听器到一个地址
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();


    loop { // loop 不断接收新的 socket 连接
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // 连接可以让我们通过 字节流，读写 redis 的 frame，连接类型被 mini-redis定义
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // 响应一个错误
        let response = Frame::Error("unimplemented".to_string());

        let response = Frame::Simple("fuck you".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
```

## 并发 Concurrency 
我们的服务有一点小问题，除了仅响应错误之外，它一次处理一个入站请求，
当一个连接被接受后，服务器将停在 accept 循环块中直到响应完成写入到 socket 中为止，

我们希望 redis 服务能够更多的并发请求，为了做到这一点，需要添加一些并发

为了同时处理连接，将为每一个入站的连接产生一个新的任务，这个连接在这个任务中处理

```rust
#[tokio::main]
async fn main() {

    // 绑定监听器到一个地址
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();


    loop { // loop 不断接收新的 socket 连接
        let (socket, _) = listener.accept().await.unwrap();
        
        // 为生一个入站 socket 连接产生一个新的任务，此 socket 连接被移动到这个新任务中并且在里面处理
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
```

## 任务
一个 tokio 任务(task) 就是一个异步的绿色线程，它们通过 async 块 tokio::spawn 来创建。
tokio::spawn 返回一个 JoinHandle 调用者可以使用该 JoinHandle 与生成的任务进行交互，
async 块可以有一个返回值，调用方可以在 JoinHandle上使用 .await 获取返回值，比如
```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
       // 做一些工作
        "return value"
    });
    
    // 做一些其他工作 
    let out = handle.await.unwrap();
    println!("GOT: {}", out);
}
```

JoinHandle 等待返回一个 Result，当任务在处理期间遇到一个错误，JoinHandle会抬一个 Err,
这种情况发生在，当任务出现  panic 或者任务在运行期间被关闭而强制取消

任务是由调度器管理的执行单元，产生的任务会提交给 tokio 的调度器，调度器可以确保在有工作要做时执行任务，
产生的任务可以在与产生它的任务在同一线程上执行，也不可在不同的线程上执行，任务产生后可以在不同的线程之间移动。

