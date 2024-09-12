use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before await

        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // rc is no longer used. It is not persisted when
        // the task yields to the scheduler
        yield_now().await;

        // yields execution back to the tokio runtime.
    });
}
