use futures::task;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::time::{Duration, Instant};
use futures::task::ArcWake;
use crate::delay::Delay;

struct MiniTokio {
    tasks: VecDeque<Task>,

    scheduled: crossbeam::channel::Receiver<Arc<Task>>,
    sender: crossbeam::channel::Sender<Arc<Task>>,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,

    executor: crossbeam::channel::Sender<Arc<Task>>,
}

// type Task = Pin<Box<dyn Future<Output=()> + Send>>;

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.
    }
}

impl MiniTokio {
    fn new() -> Self {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    // 生成一个 Future 并放入 mini-tokio 实例的任务队列中
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output=()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}


#[test]
fn test_mini_tokio_start() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1000);
        let future = Delay { when };

        let out = future.await;

        assert_eq!(out, "done");
    });

    mini_tokio.run();
}