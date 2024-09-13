use std::sync::Mutex;

/// std::sync::MutexGuard 不是 Send
/// 不能发送一个互斥锁到另外 一个线程中，并且会发生错误，
/// 因为 tokio 运行时可以在 .await 时 在线程之间移动任务
/// 你需要重组代码使得互斥锁的析构函数在 .await 之前运行
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().unwrap();

    *lock += 1;

    do_something_async().await;

    *lock += 1;
}

/// 下面也会失败，这是因为当前编译器仅根据作用域范围信息来计算一个 future 是否为 Send
/// 必须显式的使用作用域的形式
async fn increment_and_do_stuff_async(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().unwrap();

    *lock += 1;
    drop(lock);

    do_something_async().await;
}

async fn do_something_async() {}

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(1);

    // tokio::spawn(increment_and_do_stuff(&mutex));
    increment_and_do_stuff(&mutex).await;
    println!("{:?}", mutex);
}
