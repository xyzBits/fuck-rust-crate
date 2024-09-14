use bytes::Bytes;

enum Command {
    Get {
        key: String,
    },

    Set {
        key: String,
        val: Bytes,
    },
}


#[tokio::main]
async fn main() {

    // 创建一个最大容量为 32 的通道
    // tokio::sync::mpsc::channel 多生产者，单消费者
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(32);


    // move 关键字用来移动， rc 的所有权移动到 task 中去
    let manager = tokio::spawn(async move {
        let mut client = mini_redis::client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key } => {
                    client.get(&key).await.unwrap();
                }

                Command::Set { key, val } => {
                    client.set(&key, val).await.unwrap();
                }
            }
        }
    });


    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string()
        };
        tx.send(cmd).await.unwrap();
    });


    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "world".to_string(),
            val: "bar".into(),
        };

        tx2.send(cmd).await.unwrap();
    });
}
