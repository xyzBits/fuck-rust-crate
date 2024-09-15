use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(100);

    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world!");
            Poll::Ready("done")
        } else {
            // 现在忽略这一行
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
