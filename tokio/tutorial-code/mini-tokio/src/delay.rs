use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
use chrono::Local;
use crate::TIMESTAMP_FORMAT;

pub struct Delay {
    pub when: Instant,
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
            // 为当前任务克隆一个 waker 的句柄
            let waker = cx.waker().clone();
            let when = self.when;

            // 生成一个线程计时器
            // 计时器用来模拟一个阻塞等待的资源，一旦计时结束，资源准备好了，资源会通过 waker.wake() 调用通知执行器我们的任务再次被调度执行了
            thread::spawn(move || {
               let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });
            Poll::Pending


            // // 目前先忽略这行代码
            // cx.waker().wake_by_ref();
            // Poll::Pending
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