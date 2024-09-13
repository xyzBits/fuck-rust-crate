use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    // main 函数 初始化 HashMap 并传递 Arc 句柄给 process 函数，使用 Arc 可以同时从许多任务中引用 HashMap
    // 这些 hashMap 也可能在许多线程上运行，在整个 tokio中，术语 handle 句柄 用于引用提供对某些共享状态访问的值
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // Clone the handle to the hashmap
        let db = db.clone();

        println!("Accepted");

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

/// 注意，这里使用的是 std::sync::Mutex 来保护 HashMap，并不是 tokio::sync::HashMap
/// 一个常见的错误是在异步代码中无条件的使用 tokio::sync::Mutex,
/// 异步 Mutex 是一种通过调用 .await 来锁定的互斥锁
///
/// 同步的 mutex 在等待获取锁时会阻塞当前线程，反过来，将阻止其他任务的处理，
///
async fn process(socket: TcpStream, db: DB) {
    // Connection, provided by mini-redis, handles parsing frames from
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                let mut db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
