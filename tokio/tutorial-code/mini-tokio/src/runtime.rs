use chrono::Local;
use crate::TIMESTAMP_FORMAT;

#[test]
fn test_start_runtime() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

}

#[test]
fn test_start_runtime_with_thread_pool() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

}

#[test]
fn test_start_runtime_with_current_thread() {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_multi_thread_runtime() {

}

/// std::time::sleep 会阻塞整个线程
/// 而 tokio::time::sleep 则是让它所在的任务放弃 CPU 并进入调度队列等待被唤醒，它不会阻塞任何线程，
/// 它所在的线程仍然可被用来执行其他的异步任务
#[test]
fn test_tokio_sleep() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // block on 会阻塞当前线程，例如阻塞 main 线程，直到其指定的异步任务树，可能有子任务全部完成
    rt.block_on(async move {
        println!("before sleep: {}", Local::now().format(TIMESTAMP_FORMAT));

        // 只是定义了 future，此时尚未执行
        let task = tokio::time::sleep(tokio::time::Duration::from_secs(3));
        // 开始执行 task 任务，并等待它完成任务
        task.await;

        // 上面的代码执行完后，才会执行下面的代码
        println!("after sleep: {}", Local::now().format(TIMESTAMP_FORMAT));



    });


}