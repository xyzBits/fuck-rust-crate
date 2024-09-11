use mini_redis::{client, Result};

async fn say_world() {
    println!("hello world");
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // 打开连接到 mini-redis-server的链接
    // client::connect 函数功能：通过指定一个远程的地址，异步的建立一个 tcp 连接，一旦建立
    // 一个 client 处理器就会被返回，即使操作是异步的，但是我们写的代码看起来是同步的
    // 异步的唯一标识是 .await 操作符
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置 hello 的值为 world
    client.set("hello", "world".into()).await?;

    // 获取 hello 的值
    let result = client.get("hello").await?;

    println!("hello = {:?}", result);

    // 调用 say_world 函数 并没有立即执行 say_world() 函数体
    let op = say_world();

    println!("say_world has not been called");

    // 调用 .await 操作才会执行 say_world
    op.await;

    Ok(())
}
