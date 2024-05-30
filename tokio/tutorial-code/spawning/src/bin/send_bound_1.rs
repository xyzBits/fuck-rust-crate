use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {

    tokio::spawn(async {// future created by async block is not Send
       let rc = Rc::new("hello");

        // rc us used after await, it must be persisted to the task's state
        yield_now().await;

        // println!("{}", rc);
    });
}