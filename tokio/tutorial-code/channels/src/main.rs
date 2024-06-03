use bytes::Bytes;
use tokio::sync::oneshot;

enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },

    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let vec1 = vec![1, 2, 3, 4];
}
