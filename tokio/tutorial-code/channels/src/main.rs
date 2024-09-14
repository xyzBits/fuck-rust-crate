use bytes::Bytes;
use tokio::sync::oneshot;

/// 多个不同的命令在 单个通道上 复用
enum Command {
    Get {
        key: String,
        resp_sender: Responder<Option<bytes::Bytes>>,
    },

    Set {
        key: String,
        val: Bytes,
        resp_sender: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // 创建一个最大容量为 32 的通道
    // tokio::sync::mpsc::channel 多生产者，单消费者

    let (
        mut sender, // 这个 sender 用来向  redis client 的管理端发送 请求
        mut receiver, // 这个 receiver会传给 管理 redis client 的 manager，他接收消息，然后处理
                    // 每一条 command 中都会带有 一个 resp_sender，用于命令被执行后，将结果回传 到 发送方
    ) = tokio::sync::mpsc::channel(32);

    // move 关键字用来移动， rc 的所有权移动到 task 中去
    let manager = tokio::spawn(async move {
        let mut client = mini_redis::client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = receiver.recv().await {
            match cmd {
                Command::Get {
                    key,
                    resp_sender: resp,
                } => {
                    let res = client.get(&key).await;

                    // 忽略错误
                    // oneshot::Sender 上调用 send 会立即完成，而不需要 .await 操作，
                    // 这是因为 oneshot 通道 上的 send 总是立即成功或者失败，而没有任何等待
                    let _ = resp.send(res);
                }

                Command::Set {
                    key,
                    val,
                    resp_sender: resp,
                } => {
                    let res = client.set(&key, val).await;

                    // 忽略错误
                    let _ = resp.send(res);
                }
            }
        }
    });

    let tx2 = sender.clone();

    let t1 = tokio::spawn(async move {
        let (resp_sender, resp_receiver) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            resp_sender,
        };

        // 发送 get 请求
        sender.send(cmd).await.unwrap();

        // 等待响应结果
        let res = resp_receiver.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_sender, resp_receiver) = oneshot::channel();

        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp_sender,
        };

        // 发送 set 请求
        tx2.send(cmd).await.unwrap();

        // 等待响应结果
        let res = resp_receiver.await;
        println!("GOT = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
