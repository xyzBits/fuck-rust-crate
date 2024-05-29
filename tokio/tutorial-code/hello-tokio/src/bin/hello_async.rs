
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_wrold`
    let op = say_world();

    // This println! come first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`;
    op.await;
}

async fn say_world() {
    println!("world");
}