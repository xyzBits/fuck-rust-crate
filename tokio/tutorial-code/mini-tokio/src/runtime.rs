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