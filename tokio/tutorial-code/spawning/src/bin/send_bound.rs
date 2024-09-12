use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {

    tokio::spawn(async {
       // 在 .await 之前作用域中强制 rc drop
        {
            let rc = Rc::new("hello world");
            println!("{}", rc);

        }

        // rc 不再使用，当任务返回到调度器后，rc 不能再持续下去
        yield_now().await;

    });
}
