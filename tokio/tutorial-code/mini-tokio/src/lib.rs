use chrono::Local;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;

mod main_future;

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

struct Delay {
    when: Instant,
}

/// 为 Delay 类型实现 Future 特征
impl Future for Delay {
    // 关联类型 Output 是 Future 执行完成后返回值的类型
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            // 时间到了 Future 可以结束 了
            println!(
                "Hello world, time = {}",
                Local::now().format(TIMESTAMP_FORMAT)
            );

            // Future 执行结束并返回 done 字符串
            Poll::Ready("done")
        } else {
            // 目前先忽略这行代码
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 这个代码清晰的解释了如何自定义一个 Future，并指定它如何通过 poll 一步一步执行，直到最终完成返回 done 字符串
#[tokio::test]
async fn test_delay_future() {
    let when = Instant::now() + Duration::from_millis(5_000);

    let future = Delay { when };

    println!("timestamp = {}", Local::now().format(TIMESTAMP_FORMAT));

    // 运行并等待 Future 的完成
    let out = future.await;

    // 判断 Future 返回的字符串是否为 done
    assert_eq!(out, "done");
}
