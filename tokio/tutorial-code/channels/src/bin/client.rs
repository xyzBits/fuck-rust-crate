use mini_redis::client;

/// 下面的代码不能被编译，因为两个任务都需要以某种方式访问 client，而 client 没有实现 copy
/// 因此如果没有一些可以促进共享的代码，它将无法编译，另外，Client::set 需要 &mut self，这意味着
/// 需要独占的访问权限才能调用它，我们可以为每个任务打开一个连接，但这不是一个好办法，
/// 我们不能使用 std::sync::Mutex 因为 .await 需要在持有锁的情况下调用，
/// 我们可以使用 tokio::sync::Mutex 但是这样又仅允许一个进行中的请求，如果客户端 实现 pipelining
/// 异步互斥锁又不能充分利用链接了
///
#[tokio::main]
async fn main() {

    // 建立一个与 Server 的连接
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // 生成两个任务，一个获取  key，另外一个设置 key
    let task1 = tokio::spawn(async {
        let res = client.get("hello").await;
    });

    let task2 = tokio::spawn(async {
        // client.set("foo", "bar".into()).await.expect("TODO: panic message");
    });

    task1.await.unwrap();
    task2.await.unwrap();
}