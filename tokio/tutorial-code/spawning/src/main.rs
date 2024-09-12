use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // 绑定监听器到一个地址
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // loop 不断接收新的 socket 连接
        let (socket, _) = listener.accept().await.unwrap();

        // 为生一个入站 socket 连接产生一个新的任务，此 socket 连接被移动到这个新任务中并且在里面处理
        tokio::spawn(async move {
            process(socket).await;
        });
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
