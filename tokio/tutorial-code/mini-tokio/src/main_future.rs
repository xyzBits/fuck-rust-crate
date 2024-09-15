use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use crate::delay::Delay;

enum MainFuture {
    // 初始化，但永远不会被  poll
    State0,

    // 等待 Delay 运行，例如 future.await 运行
    State1(Delay),

    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match *self {
                MainFuture::State0 => {
                    let when = Instant::now() + Duration::from_millis(100);

                    let future = Delay { when };

                    *self = MainFuture::State1(future);
                }

                MainFuture::State1(ref mut delay) => match Pin::new(delay).poll(cx) {
                    Poll::Ready(out) => {
                        assert_eq!(out, "done");
                        *self = MainFuture::Terminated;
                        return Poll::Ready(());
                    }

                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },

                MainFuture::Terminated => {
                    panic!("future polled after terminated");
                }
            }
        }
    }
}
