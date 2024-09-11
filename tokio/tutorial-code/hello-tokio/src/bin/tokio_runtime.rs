/// The main function used to launch the application differs from the usual one found in
/// most of Rust's crates
// An async fn is used as we want to enter an asynchronous context. However,
// asynchronous function must be executed by a runtime. The runtime contains the
// asynchronous task scheduler, provides evented I/O, timers, etc
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        println!("hello");
    })
}
