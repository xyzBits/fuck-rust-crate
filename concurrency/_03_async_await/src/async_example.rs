use std::thread;

fn download(url: &str) {}

/// 如果是在一个小项目中简单的去下载文件，这么写没有任何问题但是一旦下载文件的并发请求多起来
/// 那一个下载任务占用一个线程的模式就太重了，会很容易成为程序的瓶颈，可以使用 async 的方式来解决
#[test]
fn test_get_two_sites_with_thread() {
    // 创建两个瓣线程执行任务
    let thread_one = thread::spawn(|| download("https:://course.rs"));
    let thread_two = thread::spawn(|| download("https:://fancy.rs"));

    // 等待两个线程的完成
    thread_one.join().unwrap();
    thread_two.join().unwrap();
}

async fn download_async(url: &str) {}

/// 下面的代码必须在一个异步运行时中运行，以便运行时 使用一定数量的线程来调度这些代码的运行
async fn get_two_sites_async() {
    // 创建两个不同的 future，你可以把 future 理解为 未来某个时刻会被执行的计划任务
    // 当两个 future 被同时执行后，它们将并发地去下载目标页面
    let future_one = download("https:://course.rs");
    let future_two = download("https:://fancy.rs");

    // 同时运行两个 future 直到 完成
    // join!(future_one, future_two);
}
