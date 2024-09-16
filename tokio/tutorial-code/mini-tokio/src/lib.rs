use std::future::Future;
use tokio::net::TcpStream;

mod delay;
mod main_future;
mod mini_tokio;

async fn my_async_fn() {
    println!("hello from async");

    // 通过 .await 创建连接
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async TCP operation complete");

    // 关闭 socket
}

/// 为什么 my_async_fn 函数可以惰性执行，直到 .await 才执行，秘密就在于
/// async fn 声明的函数返回一个 Future
/// Future 是一个实现了 std::future::Future 特征的值，该值包含了一系列的异步计算，
/// 而这个计算过程直到 .await 调用时才会执行
///
/// Rust 中的 Future 不代表一个发生在后台的计算，而是 Future 就代表了计算本身，
///
#[tokio::test]
async fn test_my_async_fn() {
    let what_is_this = my_async_fn();
    // 上面的调用不会产生任何效果

    // 执行一些其他代码。。。。
    println!("do something other");

    what_is_this.await;
    // 直到 .await 后，文本才被打印，socket 连接也被关闭
}

const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
