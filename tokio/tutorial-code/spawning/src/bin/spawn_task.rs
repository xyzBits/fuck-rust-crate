use tokio::io::AsyncSeek;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("{}", out);


    let async_fn = async {
        println!("hello async");
    };

    async_fn.await;
}