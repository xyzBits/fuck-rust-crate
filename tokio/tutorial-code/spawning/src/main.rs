use std::collections::HashMap;
use mini_redis::{Command, Connection, Frame};
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

    // 存储数据的 hash map
    // todo 有一个问题，值 不能够在连接中共享，如果另外一个 socket 尝试通过 GET 获取 另一个连接设置的值，将不会找到任何东西
    let mut db = HashMap::new();

    if let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }

            Command::Get(cmd) => {
                if let Some(key) = db.get(cmd.key()) {
                    Frame::Bulk(key.clone())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("unimplemented command: {:?}", cmd),
        };

        // 写入响应到客户端
        connection.write_frame(&response).await.unwrap();
    }
}
