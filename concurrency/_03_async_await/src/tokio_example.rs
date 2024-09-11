/// main 函数前加 async 关键字，并加 #[tokio::main] #[tokio::test] 属性，
/// 那么这个main 就会在 异步运行时中运行
#[tokio::test]
async fn test_tokio() {
    // 在异步运行时中运行任务
    tokio::spawn(async {
        // do work
    });

    // 等待任务完成
    // other_task.await;
}

#[test]
fn test_create_tokio_runtime() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from tokio");

        rt.spawn(async {
            println!("Hello from a tokio task");
            println!("in spawn");
        })
        .await
        .unwrap();
    });

    rt.spawn_blocking(|| println!("in spawn blocking"));
}
