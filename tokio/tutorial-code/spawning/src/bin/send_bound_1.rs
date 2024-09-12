use std::rc::Rc;
use tokio::main;
use tokio::task::yield_now;

#[main]
async fn main() {
    tokio::spawn(async {
        /// future cannot be sent between threads safely
        /// Rc 并没有实现 Send 所以不能跨线程移动，必须  提前 drop 否则编译无法通过
       let rc = Rc::new("hello world");


        // rc 不能跨线程移动，所以要先 drop
        drop(rc);
        // rc 在 .await 后继续使用，它必须持久化后 task 的状态中才行

        yield_now().await;
        // println!("{}", rc);

    });
}