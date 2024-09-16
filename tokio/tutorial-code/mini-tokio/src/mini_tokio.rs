use crate::TIMESTAMP_FORMAT;
use futures::future::BoxFuture;
use futures::task;
use futures::task::ArcWake;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

struct MiniTokio {
    // 接收预定任务，当任务被调度时，相关的 future 就准备好运行
    // 这通常发生在任务使用的资源被准备好时，
    // 例如，socket 收到数据，并且 read 成功
    scheduled: crossbeam::channel::Receiver<Arc<Task>>,

    // 调度的另一半，发送任务
    sender: crossbeam::channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = crossbeam::channel::unbounded();

        MiniTokio { scheduled, sender }
    }

    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        CURRENT.with(|cell| {
            *cell.borrow_mut() = Some(self.sender.clone());
        });

        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    CURRENT.with(|cell| {
        let borrow = cell.borrow();
        let sender = borrow.as_ref().unwrap();
        Task::spawn(future, sender);
    });
}

async fn delay(dur: Duration) {
    struct Delay {
        when: Instant,

        waker: Option<Arc<Mutex<Waker>>>,
    }

    impl Future for Delay {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();

                if !waker.will_wake(cx.waker()) {
                    *waker = cx.waker().clone();
                }
            } else {
                let when = self.when;
                let waker = Arc::new(Mutex::new(cx.waker().clone()));
                self.waker = Some(waker.clone());

                thread::spawn(move || {
                    let now = Instant::now();

                    if now < when {
                        thread::sleep(when - now);
                    }

                    let waker = waker.lock().unwrap();
                    waker.wake_by_ref();
                });
            }

            if Instant::now() >= self.when {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    let future = Delay {
        when: Instant::now() + dur,
        waker: None,
    };

    future.await;
}
thread_local! {
    static CURRENT: RefCell<Option<crossbeam::channel::Sender<Arc<Task>>>> = RefCell::new(None);
}

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,

    executor: crossbeam::channel::Sender<Arc<Task>>,
}

impl Task {
    fn spawn<F>(future: F, sender: &crossbeam::channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

    fn poll(self: &Arc<Self>) {
        let waker = task::waker(self.clone());

        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let _ = arc_self.executor.send(arc_self.clone());
    }
}

#[test]
fn test_mini_tokio() {
    let mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        spawn(async {
            delay(Duration::from_secs(1)).await;
            println!(
                "world, time = {}",
                chrono::Local::now().format(TIMESTAMP_FORMAT)
            );
        });

        spawn(async {
            println!(
                "hello, time = {}",
                chrono::Local::now().format(TIMESTAMP_FORMAT)
            );
        });

        delay(Duration::from_millis(200)).await;

        std::process::exit(0);
    });

    mini_tokio.run();
}
