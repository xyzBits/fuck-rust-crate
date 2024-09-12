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